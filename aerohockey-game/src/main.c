#include "graphics/window.h"
#include "graphics/renderer.h"
#include <stdio.h>
#include <stdbool.h>

#define SCREEN_WIDTH 800
#define SCREEN_HEIGHT 600

int main() {
    Window window;
    if (init_window(&window, "Аэрохоккей", SCREEN_WIDTH, SCREEN_HEIGHT) != 0) {
        printf("Не удалось инициализировать окно или рендерер.\n");
        return 1;
    }

    bool running = true;
    while (running) {
        SDL_Event event;
        while (SDL_PollEvent(&event)) {
            if (event.type == SDL_QUIT) {
                running = false;
            }
        }

        clear_screen(window.renderer);  // Очищаем экран
        set_draw_color(window.renderer, 255, 255, 255, 255); // Белый цвет для рисования
        // Здесь можно рисовать объекты, например, прямоугольники:
        // SDL_RenderFillRect(window.renderer, &rect);

        present_renderer(window.renderer); // Обновляем экран
    }

    destroy_window(&window); // Закрываем окно и рендерер
    return 0;
}