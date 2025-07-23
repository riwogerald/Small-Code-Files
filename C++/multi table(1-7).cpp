#include <Windows.h>
#include <vcclr.h>
#include <msclr\marshal_cppstd.h>

using namespace System;
using namespace System::ComponentModel;
using namespace System::Collections;
using namespace System::Windows::Forms;
using namespace System::Data;
using namespace System::Drawing;

public ref class MultiplicationTableForm : public System::Windows::Forms::Form
{
public:
    MultiplicationTableForm(void)
    {
        InitializeComponent();
    }

protected:
    ~MultiplicationTableForm()
    {
        if (components)
        {
            delete components;
        }
    }

private:
    System::Windows::Forms::PictureBox^ pictureBox1;
    System::ComponentModel::Container^ components;

    void InitializeComponent(void)
    {
        this->pictureBox1 = (gcnew System::Windows::Forms::PictureBox());
        (cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->pictureBox1))->BeginInit();
        this->SuspendLayout();
        
        // pictureBox1
        this->pictureBox1->BackColor = System::Drawing::Color::White;
        this->pictureBox1->BorderStyle = System::Windows::Forms::BorderStyle::FixedSingle;
        this->pictureBox1->Location = System::Drawing::Point(12, 12);
        this->pictureBox1->Name = L"pictureBox1";
        this->pictureBox1->Size = System::Drawing::Size(600, 400);
        this->pictureBox1->TabIndex = 0;
        this->pictureBox1->TabStop = false;
        this->pictureBox1->Paint += gcnew System::Windows::Forms::PaintEventHandler(this, &MultiplicationTableForm::pictureBox1_Paint);
        
        // MultiplicationTableForm
        this->AutoScaleDimensions = System::Drawing::SizeF(6, 13);
        this->AutoScaleMode = System::Windows::Forms::AutoScaleMode::Font;
        this->ClientSize = System::Drawing::Size(624, 424);
        this->Controls->Add(this->pictureBox1);
        this->Name = L"MultiplicationTableForm";
        this->Text = L"Multiplication Table Generator";
        this->FormBorderStyle = System::Windows::Forms::FormBorderStyle::FixedSingle;
        this->MaximizeBox = false;
        
        (cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->pictureBox1))->EndInit();
        this->ResumeLayout(false);
    }

    void pictureBox1_Paint(System::Object^ sender, System::Windows::Forms::PaintEventArgs^ e)
    {
        Graphics^ g = e->Graphics;
        
        // Set up fonts and brushes
        Font^ headerFont = gcnew Font("Arial", 12, FontStyle::Bold);
        Font^ dataFont = gcnew Font("Arial", 11, FontStyle::Regular);
        Brush^ blackBrush = gcnew SolidBrush(Color::Black);
        Brush^ headerBrush = gcnew SolidBrush(Color::DarkBlue);
        Pen^ gridPen = gcnew Pen(Color::Gray, 1);
        
        // Define cell dimensions
        int cellWidth = 70;
        int cellHeight = 35;
        int startX = 20;
        int startY = 20;
        
        // Draw title
        Font^ titleFont = gcnew Font("Arial", 16, FontStyle::Bold);
        g->DrawString("Multiplication Table", titleFont, headerBrush, startX, startY - 5);
        
        // Adjust starting position for the table
        startY += 40;
        
        // Draw header row (column numbers)
        g->DrawString("*", headerFont, headerBrush, startX + 25, startY + 8);
        for (int col = 1; col <= 7; col++)
        {
            int x = startX + col * cellWidth;
            int y = startY;
            
            // Draw cell border
            g->DrawRectangle(gridPen, x, y, cellWidth, cellHeight);
            
            // Draw column header number
            String^ colText = col.ToString();
            g->DrawString(colText, headerFont, headerBrush, x + 30, y + 8);
        }
        
        // Draw data rows
        for (int row = 1; row <= 7; row++)
        {
            int y = startY + row * cellHeight;
            
            // Draw row header
            String^ rowText = row.ToString();
            g->DrawString(rowText, headerFont, headerBrush, startX + 25, y + 8);
            
            // Draw data cells for this row
            for (int col = 1; col <= 7; col++)
            {
                int x = startX + col * cellWidth;
                
                // Draw cell border
                g->DrawRectangle(gridPen, x, y, cellWidth, cellHeight);
                
                // Calculate and draw multiplication result
                int result = row * col;
                String^ resultText = result.ToString();
                
                // Center the text in the cell
                int textX = x + (cellWidth - (int)(g->MeasureString(resultText, dataFont).Width)) / 2;
                int textY = y + 8;
                
                g->DrawString(resultText, dataFont, blackBrush, textX, textY);
            }
        }
        
        // Draw border around header column
        g->DrawRectangle(gridPen, startX, startY, cellWidth, cellHeight);
        for (int row = 1; row <= 7; row++)
        {
            int y = startY + row * cellHeight;
            g->DrawRectangle(gridPen, startX, y, cellWidth, cellHeight);
        }
        
        // Clean up resources
        delete headerFont;
        delete dataFont;
        delete titleFont;
        delete blackBrush;
        delete headerBrush;
        delete gridPen;
    }
};

[STAThreadAttribute]
int main(cli::array<System::String^>^ args)
{
    Application::EnableVisualStyles();
    Application::SetCompatibleTextRenderingDefault(false);
    Application::Run(gcnew MultiplicationTableForm());
    return 0;
}