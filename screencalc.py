import tkinter


def main():
    tk = tkinter.Tk()
    tk.title("Screen Calculator")
    tk.geometry("400x200")

    # Create the widgets (Pixel Width, Pixel Height, Diagonal Size)
    pixel_wxh_label = tkinter.Label(tk, text="Pixel Width x Height:")
    pixel_wxh_label.pack()
    pixel_wxh_entry = tkinter.Entry(tk)
    pixel_wxh_entry.pack()

    diagonal_size_label = tkinter.Label(tk, text="Diagonal Size (inch):")
    diagonal_size_label.pack()
    diagonal_size_entry = tkinter.Entry(tk)
    diagonal_size_entry.pack()

    result_label = tkinter.Label(tk, text="Result:")
    result_label.pack()

    # Create the calculate button
    def calculate():
        # result: cm width, cm height
        pixel_wxh = pixel_wxh_entry.get().split("x")
        diagonal_size_inch = float(diagonal_size_entry.get())
        diagonal_size = diagonal_size_inch * 2.54

        pixel_w = int(pixel_wxh[0])
        pixel_h = int(pixel_wxh[1])
        pixel_size = (pixel_w ** 2 + pixel_h ** 2) ** 0.5
        pixel_density = pixel_size / diagonal_size
        cm_w = pixel_w / pixel_density
        cm_h = pixel_h / pixel_density
        result_label.config(text=f"Result: {cm_w:.2f} x {cm_h:.2f} cm")

    calculate_button = tkinter.Button(tk, text="Calculate", command=calculate)
    calculate_button.pack()

    tk.mainloop()


if __name__ == '__main__':
    main()
