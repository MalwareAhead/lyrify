# Lyrify: Lyrics to PPTX Converter

A Python application to convert lyrics stored in `.txt` files into visually appealing PowerPoint presentations. The app takes lyrics from a text file, creates slides, and saves the presentation in the `.pptx` format.

## Features

- **Drag and Drop Support**: Drag and drop `.txt` files into the window to start the conversion process.
- **File Selection**: Alternatively, select text files from your computer via a file dialog.
- **Custom Formatting**: Automatically formats the lyrics into a slide layout with centered, large text, and a custom black background.
- **Preview Text**: Displays the first line of the next slide as a preview at the bottom of each slide.
- **Final Empty Slide**: Adds a final empty slide for a clean finish.
- **Save to Directory**: Saves the presentations in a dedicated `presentations` folder.

## Requirements

- Python 3.x
- Install the required libraries by running:

```bash
pip install python-pptx tkinter tkinterdnd2
```

## Usage

1. **Drag and Drop**: Drag `.txt` files into the designated area in the window.
2. **File Selection**: Click anywhere in the window to open the file dialog and select `.txt` files manually.
3. The program will automatically create a PowerPoint presentation for each text file dropped or selected.

### .txt File Format

The text file should contain lyrics or any other content you wish to present, with each slide's content separated by two newlines (`\n\n`). For example:

```
First line of the first slide

Second line of the first slide

---
First line of the second slide

Second line of the second slide
```

Each block of text will be placed on a new slide.

## Output

- The presentations will be saved in the `presentations` folder in the same directory as the script.
- Each presentation is named after the original `.txt` file (with `.pptx` extension).

## How It Works

- The application reads the `.txt` files line by line.
- Each block of text separated by two newlines is treated as the content for a slide.
- The slides are created with centered text, white font on a black background.
- A preview of the next slide is added to the bottom of each slide for better context.
- After processing, the presentation is saved as a `.pptx` file.

## Screenshots

<img width="603" alt="image" src="https://github.com/user-attachments/assets/7242354f-d41b-4425-ae09-b5ebb3a31c02" />


## Contributing

If you find any issues or want to improve the project, feel free to create a pull request or report an issue. Contributions are always welcome!

## License

This project is open-source and available under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

Feel free to modify or add additional details depending on your needs. Let me know if you’d like any changes or more details!
