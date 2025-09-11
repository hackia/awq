#include <ftxui/component/screen_interactive.hpp>
#include <ftxui/dom/elements.hpp>

#include "ftxui/component/component.hpp"
using namespace ftxui;

int main() {
  auto screen = ScreenInteractive::Fullscreen();

  auto line1 = hbox({
    text("Consequences ") | color(Color::White) | italic | bold | center,
    text("follow every step and every step begins with a ") | color(Color::LightGreen) | dim | italic | center,
    text("choice") | color(Color::White) | bold | italic | center,
  });

  auto doc = vbox({
               filler(),
               hbox({filler(), vbox({line1}) | center, filler()}),
               filler(),
             }) | bgcolor(Color::Black);

  screen.Loop(Renderer([&] { return doc; }));
}
