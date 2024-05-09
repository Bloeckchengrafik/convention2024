import os
import threading
import pygame


def cvimage_to_pygame(image):
    """Convert cvimage into a pygame image"""
    return pygame.image.frombuffer(
        image.tobytes(),
        image.shape[1::-1],
        "RGB"
    )


class DisplaySystem(threading.Thread):
    def __init__(self):
        super().__init__()
        self.daemon = True
        self.left = None
        self.right = None
        self.distance = 0
        self.net_clock = pygame.time.Clock()
        self.screen_clock = pygame.time.Clock()
        self.running = True

    def set_cv_data(self, left, right, distance):
        self.left = cvimage_to_pygame(left)
        self.right = cvimage_to_pygame(right)
        self.distance = distance
        self.net_clock.tick()

    def handle_key(self, key):
        if key == pygame.K_ESCAPE:
            self.running = False

    def draw(self, screen, font):
        # draw them next to each other centered on the screen
        screen_width = screen.get_width()
        screen_height = screen.get_height()
        left_width = self.left.get_width()
        left_height = self.left.get_height()
        right_width = self.right.get_width()
        right_height = self.right.get_height()
        left_top = (screen_height - left_height) // 2
        right_top = (screen_height - right_height) // 2
        left_left = (screen_width - left_width - right_width) // 2
        right_left = left_left + left_width

        screen.blit(self.left, (left_left, left_top))
        screen.blit(self.right, (right_left, right_top))

        # draw fps on top left corner
        fps = self.net_clock.get_fps()
        screen_fps = self.screen_clock.get_fps()
        text = font.render(f"Net FPS: {fps:.2f} | Screen FPS: {screen_fps:.2f}", True, (255, 255, 255))
        screen.blit(text, (10, 10))

    def draw_error(self, screen, text, distance=100):
        """
        Error should be drawn centered on the screen twice
        so that it is next to each other.
        """

        text_width = text.get_width()
        screen_height = screen.get_height()
        top = (screen_height - text.get_height()) // 2
        center = (screen.get_width() - text_width) // 2
        screen.blit(text, (center - text_width / 2 - distance, top))
        screen.blit(text, (center + text_width / 2 + distance, top))

    def run(self):
        """
        Run the display system (pygame screen)

        Full-screen display of left and right images (centered) with distance (in pixels) of space between them
        """

        pygame.init()
        screen = pygame.display.set_mode((1920, 1080))
        pygame.display.set_caption("ftVR iodev")
        screen.fill((0, 0, 0))
        font = pygame.font.Font(None, 36)
        text = font.render("Images not available", True, (255, 255, 255))
        while self.running:
            screen.fill((0, 0, 0))
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    self.running = False
                if event.type == pygame.KEYDOWN:
                    self.handle_key(event.key)
            if self.left is not None and self.right is not None:
                self.draw(screen, font)
            else:
                self.draw_error(screen, text)
            pygame.display.flip()
            self.screen_clock.tick(60)
        pygame.quit()
        os._exit(0)
