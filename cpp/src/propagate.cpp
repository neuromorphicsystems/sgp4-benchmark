#include "../third_party/json.hpp"
#include "../third_party/sgp4ext.h"
#include "../third_party/sgp4unit.h"
#include <array>
#include <chrono>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

struct ymd_s_ns {
    uint32_t ymd;
    uint32_t hms;
    uint32_t ns;

    ymd_s_ns(const std::string& representation) :
        ymd(static_cast<uint32_t>((std::stoul(representation.substr(0, 4)) << 9)
            | (std::stoul(representation.substr(5, 2)) << 5)
            | std::stoul(representation.substr(8, 2)))),
        hms(static_cast<uint32_t>((std::stoul(representation.substr(11, 2)) << 12)
            | (std::stoul(representation.substr(14, 2)) << 6)
            | std::stoul(representation.substr(17, 2)))),
        ns(static_cast<uint32_t>(std::stod(representation.substr(19)) * 1e9))
    {}
};

struct prediction {
    std::array<double, 3> position;
    std::array<double, 3> velocity;
};

enum Classification {
    Unclassified,
    Classified,
    Secret,
};

struct elements {
    std::string object_name;
    std::string international_designator;
    uint64_t norad_id;
    Classification classification;
    ymd_s_ns datetime;
    double mean_motion_dot;
    double mean_motion_ddot;
    double drag_term;
    uint64_t element_set_number;
    double inclination;
    double right_ascension;
    double eccentricity;
    double argument_of_perige;
    double mean_anomaly;
    double mean_motion;
    uint64_t revolution_number;
    uint8_t ephemeris_type;
};

int main(int argc, char* argv[]) {
    assert((argc == 3));
    std::ifstream input(argv[1]);
    assert((input.good()));
    nlohmann::json json;
    input >> json;
    std::vector<elements> omms;
    for (const auto& omm : json) {
        omms.push_back(elements {
            omm["OBJECT_NAME"],
            omm["OBJECT_ID"],
            omm["NORAD_CAT_ID"],
            Classification::Unclassified,
            ymd_s_ns(omm["EPOCH"]),
            omm["MEAN_MOTION_DOT"],
            omm["MEAN_MOTION_DDOT"],
            omm["BSTAR"],
            omm["ELEMENT_SET_NO"],
            omm["INCLINATION"],
            omm["RA_OF_ASC_NODE"],
            omm["ECCENTRICITY"],
            omm["ARG_OF_PERICENTER"],
            omm["MEAN_ANOMALY"],
            omm["MEAN_MOTION"],
            omm["REV_AT_EPOCH"],
            omm["EPHEMERIS_TYPE"],
        });
    }
    std::vector<prediction> predictions;
    predictions.reserve(omms.size() * 1440);
    const double deg2rad = pi / 180.0;
    const double xpdotp = 1440.0 / (2.0 * pi);
    double tumin, mu, radiusearthkm, xke, j2, j3, j4, j3oj2;
    getgravconst(SGP4_GEOPOTENTIAL, tumin, mu, radiusearthkm, xke, j2, j3, j4, j3oj2);
    const auto start = std::chrono::high_resolution_clock::now();
    for (const auto& omm : omms) {
        elsetrec satrec;
        jday(
            omm.datetime.ymd >> 9,
            (omm.datetime.ymd >> 5) & 0b1111,
            omm.datetime.ymd & 0b11111,
            omm.datetime.hms >> 12,
            (omm.datetime.hms >> 6) & 0b111111,
            static_cast<double>(omm.datetime.hms & 0b111111)
                + static_cast<double>(omm.datetime.ns) / 1e9,
            satrec.jdsatepoch);
        satrec.satnum = static_cast<int>(omm.norad_id);
        satrec.no = omm.mean_motion / xpdotp;
        satrec.bstar = omm.drag_term;
        satrec.a = std::pow(satrec.no * tumin, (-2.0 / 3.0));
        satrec.ndot = omm.mean_motion_dot / (xpdotp * 1440.0);
        satrec.nddot = omm.mean_motion_ddot / (xpdotp * 1440.0 * 1440);
        satrec.inclo = omm.inclination * deg2rad;
        satrec.nodeo = omm.right_ascension * deg2rad;
        satrec.argpo = omm.argument_of_perige * deg2rad;
        satrec.mo = omm.mean_anomaly * deg2rad;
        satrec.ecco = omm.eccentricity;
        satrec.alta = satrec.a * (1.0 + satrec.ecco) - 1.0;
        satrec.altp = satrec.a * (1.0 - satrec.ecco) - 1.0;
        assert((sgp4init(
            SGP4_GEOPOTENTIAL,
            SGP4_MODE,
            static_cast<int>(satrec.satnum),
            satrec.jdsatepoch - 2433281.5,
            satrec.bstar,
            satrec.ecco,
            satrec.argpo,
            satrec.inclo,
            satrec.mo,
            satrec.no,
            satrec.nodeo,
            satrec)));
        for (uint64_t t = 0; t < 1440; ++t) {
            predictions.emplace_back();
            assert((sgp4(
                SGP4_GEOPOTENTIAL,
                satrec,
                static_cast<double>(t),
                predictions.back().position.data(),
                predictions.back().velocity.data())));
        }
    }
    const auto duration = std::chrono::high_resolution_clock::now() - start;
    std::cout << std::chrono::duration_cast<std::chrono::microseconds>(duration).count() << std::endl;
    std::ofstream output(argv[2]);
    assert((output.good()));
    for (const auto& result : predictions) {
        output.write(
            reinterpret_cast<const char*>(result.position.data()),
            sizeof(decltype(result.position)::value_type) * result.position.size());
        output.write(
            reinterpret_cast<const char*>(result.velocity.data()),
            sizeof(decltype(result.velocity)::value_type) * result.velocity.size());
    }
    return 0;
}
