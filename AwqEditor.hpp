#pragma once

#include <ftxui/screen/screen.hpp>
#include <filesystem>
#include <fstream>
#include <string>
#include <vector>
#include <ftxui/component/component.hpp>
#include <stdexcept>
#include <ftxui/dom/elements.hpp>

namespace awq {
    class AwqEditor {
    public:
        explicit AwqEditor(const std::string &filename);

         ftxui::Component render() const;

    private:
        std::string filename;
    };
}
