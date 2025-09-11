#include "AwqEditor.hpp"
using namespace awq;
using namespace ftxui;

int main(const int argc, const char **argv) {
    if (argc > 1) {
        const auto e = new AwqEditor(argv[1]);
        const auto content = e->render();
        delete e;
        return 0;
    }
    return 0;
}
