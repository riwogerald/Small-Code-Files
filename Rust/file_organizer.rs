use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::io;

struct FileOrganizer {
    source_dir: PathBuf,
    target_dir: PathBuf,
    file_types: HashMap<String, String>,
}

impl FileOrganizer {
    fn new(source: &str, target: &str) -> Self {
        let mut file_types = HashMap::new();
        
        // Define file type mappings
        file_types.insert("txt".to_string(), "Documents".to_string());
        file_types.insert("doc".to_string(), "Documents".to_string());
        file_types.insert("docx".to_string(), "Documents".to_string());
        file_types.insert("pdf".to_string(), "Documents".to_string());
        file_types.insert("rtf".to_string(), "Documents".to_string());
        
        file_types.insert("jpg".to_string(), "Images".to_string());
        file_types.insert("jpeg".to_string(), "Images".to_string());
        file_types.insert("png".to_string(), "Images".to_string());
        file_types.insert("gif".to_string(), "Images".to_string());
        file_types.insert("bmp".to_string(), "Images".to_string());
        file_types.insert("svg".to_string(), "Images".to_string());
        
        file_types.insert("mp3".to_string(), "Audio".to_string());
        file_types.insert("wav".to_string(), "Audio".to_string());
        file_types.insert("flac".to_string(), "Audio".to_string());
        file_types.insert("aac".to_string(), "Audio".to_string());
        file_types.insert("ogg".to_string(), "Audio".to_string());
        
        file_types.insert("mp4".to_string(), "Videos".to_string());
        file_types.insert("avi".to_string(), "Videos".to_string());
        file_types.insert("mkv".to_string(), "Videos".to_string());
        file_types.insert("mov".to_string(), "Videos".to_string());
        file_types.insert("wmv".to_string(), "Videos".to_string());
        
        file_types.insert("zip".to_string(), "Archives".to_string());
        file_types.insert("rar".to_string(), "Archives".to_string());
        file_types.insert("7z".to_string(), "Archives".to_string());
        file_types.insert("tar".to_string(), "Archives".to_string());
        file_types.insert("gz".to_string(), "Archives".to_string());
        
        file_types.insert("exe".to_string(), "Programs".to_string());
        file_types.insert("msi".to_string(), "Programs".to_string());
        file_types.insert("deb".to_string(), "Programs".to_string());
        file_types.insert("dmg".to_string(), "Programs".to_string());
        
        FileOrganizer {
            source_dir: PathBuf::from(source),
            target_dir: PathBuf::from(target),
            file_types,
        }
    }

    fn get_file_category(&self, extension: &str) -> String {
        self.file_types
            .get(&extension.to_lowercase())
            .cloned()
            .unwrap_or_else(|| "Others".to_string())
    }

    fn create_directory_if_not_exists(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !path.exists() {
            fs::create_dir_all(path)?;
            println!("Created directory: {}", path.display());
        }
        Ok(())
    }

    fn scan_directory(&self) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        
        if !self.source_dir.exists() {
            return Err(format!("Source directory does not exist: {}", self.source_dir.display()).into());
        }

        for entry in fs::read_dir(&self.source_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                files.push(path);
            }
        }

        Ok(files)
    }

    fn organize_files(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
        let files = self.scan_directory()?;
        
        if files.is_empty() {
            println!("No files found in source directory.");
            return Ok(());
        }

        let mut organization_plan: HashMap<String, Vec<PathBuf>> = HashMap::new();
        
        // Analyze files and create organization plan
        for file_path in &files {
            let extension = file_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            let category = self.get_file_category(&extension);
            organization_plan
                .entry(category)
                .or_insert_with(Vec::new)
                .push(file_path.clone());
        }

        // Display organization plan
        println!("\n=== File Organization Plan ===");
        for (category, files) in &organization_plan {
            println!("\n📁 {} ({} files):", category, files.len());
            for file in files {
                if let Some(filename) = file.file_name() {
                    println!("  • {}", filename.to_string_lossy());
                }
            }
        }

        if dry_run {
            println!("\n🔍 This was a dry run. No files were moved.");
            return Ok(());
        }

        // Confirm before proceeding
        print!("\nProceed with file organization? (y/n): ");
        io::Write::flush(&mut io::stdout()).unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Operation cancelled.");
            return Ok(());
        }

        // Create target directory structure and move files
        self.create_directory_if_not_exists(&self.target_dir)?;
        
        let mut moved_count = 0;
        let mut error_count = 0;

        for (category, files) in organization_plan {
            let category_dir = self.target_dir.join(&category);
            self.create_directory_if_not_exists(&category_dir)?;

            for file_path in files {
                if let Some(filename) = file_path.file_name() {
                    let target_path = category_dir.join(filename);
                    
                    match fs::rename(&file_path, &target_path) {
                        Ok(_) => {
                            println!("✓ Moved: {} → {}", 
                                   file_path.display(), 
                                   target_path.display());
                            moved_count += 1;
                        }
                        Err(e) => {
                            println!("✗ Failed to move {}: {}", file_path.display(), e);
                            error_count += 1;
                        }
                    }
                }
            }
        }

        println!("\n=== Organization Complete ===");
        println!("Files moved: {}", moved_count);
        println!("Errors: {}", error_count);

        Ok(())
    }

    fn display_statistics(&self) -> Result<(), Box<dyn std::error::Error>> {
        let files = self.scan_directory()?;
        
        if files.is_empty() {
            println!("No files found in source directory.");
            return Ok(());
        }

        let mut stats: HashMap<String, usize> = HashMap::new();
        let mut total_size = 0u64;

        for file_path in &files {
            let extension = file_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("no_extension")
                .to_lowercase();
            
            let category = self.get_file_category(&extension);
            *stats.entry(category).or_insert(0) += 1;

            if let Ok(metadata) = fs::metadata(file_path) {
                total_size += metadata.len();
            }
        }

        println!("\n=== Directory Statistics ===");
        println!("Source: {}", self.source_dir.display());
        println!("Total files: {}", files.len());
        println!("Total size: {:.2} MB", total_size as f64 / 1_048_576.0);
        
        println!("\nFile distribution by category:");
        let mut sorted_stats: Vec<_> = stats.iter().collect();
        sorted_stats.sort_by(|a, b| b.1.cmp(a.1));
        
        for (category, count) in sorted_stats {
            let percentage = (*count as f64 / files.len() as f64) * 100.0;
            println!("  {}: {} files ({:.1}%)", category, count, percentage);
        }

        Ok(())
    }
}

fn main() {
    println!("📁 File Organizer");
    println!("=================");

    print!("Enter source directory path: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut source = String::new();
    io::stdin().read_line(&mut source).expect("Failed to read input");
    let source = source.trim();

    print!("Enter target directory path: ");
    io::Write::flush(&mut io::stdout()).unwrap();
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read input");
    let target = target.trim();

    let organizer = FileOrganizer::new(source, target);

    loop {
        println!("\nOptions:");
        println!("1. Show directory statistics");
        println!("2. Preview organization (dry run)");
        println!("3. Organize files");
        println!("4. Exit");
        print!("Choose an option (1-4): ");
        io::Write::flush(&mut io::stdout()).unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => {
                if let Err(e) = organizer.display_statistics() {
                    println!("Error: {}", e);
                }
            }
            "2" => {
                if let Err(e) = organizer.organize_files(true) {
                    println!("Error: {}", e);
                }
            }
            "3" => {
                if let Err(e) = organizer.organize_files(false) {
                    println!("Error: {}", e);
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice! Please enter 1-4.");
            }
        }
    }
}