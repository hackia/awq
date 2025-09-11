#include "AwqEditor.hpp"

#include <filesystem>

#include "ftxui/component/screen_interactive.hpp"
using namespace std;
namespace fs = std::filesystem;
using namespace ftxui;
#include "ftxui/component/component.hpp"
#include "ftxui/component/event.hpp"
#include "ftxui/dom/elements.hpp"

namespace awq {
    AwqEditor::AwqEditor(const string &filename) : filename(filename) {
    }

    Component AwqEditor::render() const {
        auto container = Container::Vertical({});
        std::string message = "'ESC' to quit)";
        auto screen = ScreenInteractive::Fullscreen();
        if (!fs::exists(filename)) {
            std::ofstream file(filename.c_str());
            if (!file.is_open()) {
                throw std::runtime_error("Cannot open file");
            }
            file << "new file\n";
            file.close();
        }
        fstream file(filename, ios::in);
        string line;
        while (getline(file, line)) {
            container->Add(Renderer([line] {
                return text(line);
            }));
        }
        file.close();
        auto final_renderer = Renderer(container, [container] {
            return container->Render();
        });
        auto component = final_renderer | CatchEvent([&](const Event &event) {
            if (event == Event::Escape) {
                screen.Clear();
                screen.Exit();
                return true;
            }
            if (event.is_character()) {
                return true;
            }
            return false;
        });

        screen.Loop(component);

        return final_renderer;
    }
}
