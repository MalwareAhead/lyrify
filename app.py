import tkinter as tk
from tkinter import ttk
import os
from pptx import Presentation
from pptx.util import Pt
from pptx.enum.text import PP_ALIGN
from pptx.dml.color import RGBColor
import tkinter.messagebox as messagebox
from tkinter import filedialog
from tkinterdnd2 import DND_FILES, TkinterDnD


class LyricsToPPTXConverter:
    def __init__(self):
        self.window = TkinterDnD.Tk()
        self.window.title("Lyrics to PPTX Converter")
        self.window.geometry("600x400")

        # Create output directory
        self.output_dir = "presentations"
        if not os.path.exists(self.output_dir):
            os.makedirs(self.output_dir)

        self.setup_ui()

    def setup_ui(self):
        # Create drop frame
        self.drop_frame = ttk.LabelFrame(self.window, text="Drop Files Here")
        self.drop_frame.pack(padx=20, pady=20, fill=tk.BOTH, expand=True)

        # Label for instructions
        self.label = ttk.Label(self.drop_frame,
                               text="Drop .txt files here\nor click to select files")
        self.label.pack(padx=20, pady=50)

        # Configure drag and drop
        self.drop_frame.drop_target_register(DND_FILES)
        self.drop_frame.dnd_bind('<<Drop>>', self.handle_drop)
        self.drop_frame.bind("<Button-1>", self.select_files)

    def select_files(self, event=None):
        files = filedialog.askopenfilenames(
            filetypes=[("Text Files", "*.txt")]
        )
        if files:
            self.process_files(files)

    def handle_drop(self, event):
        files = event.data.split()
        # Clean file paths (remove curly braces if present)
        files = [f.strip('{}') for f in files]
        text_files = [f for f in files if f.lower().endswith('.txt')]
        if text_files:
            self.process_files(text_files)

    def process_files(self, files):
        for file_path in files:
            try:
                self.create_presentation(file_path)
                messagebox.showinfo("Success",
                                    f"Presentation created for {os.path.basename(file_path)}")
            except Exception as e:
                messagebox.showerror("Error",
                                     f"Error processing {os.path.basename(file_path)}: {str(e)}")

    def create_presentation(self, txt_file):
        # Read lyrics
        with open(txt_file, 'r', encoding='utf-8') as file:
            content = file.read()

        # Split into slides
        slides_content = [slide.strip() for slide in content.split('\n\n')]

        # Create presentation
        prs = Presentation()
        prs.slide_width = Pt(1920)
        prs.slide_height = Pt(1080)

        # Find longest line for font sizing
        longest_line = max([len(line) for slide in slides_content
                            for line in slide.split('\n')])
        base_font_size = min(100, int(1800 / longest_line))

        # Create slides
        for slide_text in slides_content:
            slide = prs.slides.add_slide(prs.slide_layouts[6])

            # Set black background
            background = slide.background
            fill = background.fill
            fill.solid()
            fill.fore_color.rgb = RGBColor(0, 0, 0)

            # Add text box
            text_box = slide.shapes.add_textbox(
                left=Pt(100),
                top=Pt(100),
                width=Pt(1720),
                height=Pt(880)
            )

            # Format text
            text_frame = text_box.text_frame
            text_frame.text = slide_text
            paragraph = text_frame.paragraphs[0]
            paragraph.alignment = PP_ALIGN.CENTER

            # Apply formatting to each line
            for line in text_frame.paragraphs:
                line.font.name = 'Arial'
                line.font.size = Pt(base_font_size)
                line.font.color.rgb = RGBColor(255, 255, 255)

        # Save presentation
        output_filename = os.path.splitext(os.path.basename(txt_file))[0] + '.pptx'
        output_path = os.path.join(self.output_dir, output_filename)
        prs.save(output_path)

    def run(self):
        self.window.mainloop()


if __name__ == "__main__":
    app = LyricsToPPTXConverter()
    app.run()