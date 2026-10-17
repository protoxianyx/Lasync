use std::{fs, io, path::Path};

fn main() {
    println!("Lasync starting...");
}

fn sync_file(local_path: &Path, drive_path: &Path) -> io::Result<()> {
    if local_path.exists() && !drive_path.exists() {
        println!("New Local File! Copying to Drive: {:?}", drive_path);
        fs::copy(local_path, drive_path)?;
        return Ok(());
    }

    if !local_path.exists() && drive_path.exists() {
        println!("New Drive File! Copying to Local: {:?}", local_path);
        fs::copy(local_path, drive_path)?;
        return Ok(());
    }

    if local_path.exists() && drive_path.exists() {
        let local_time = fs::metadata(local_path)?.modified()?;
        let drive_time = fs::metadata(drive_path)?.modified()?;
        if local_time > drive_time {
            println!("Local is newer -> Updating Drive");
            fs::copy(local_path, drive_path)?;
        } else if drive_time > local_time {
            println!("Drive is newer -> Updating Local");
            fs::copy(drive_path, local_path)?;
        } else {
            println!("Both files are already in sync.");
        }
    }

    Ok(())
}
