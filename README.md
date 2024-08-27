## Memoir: Your Ultimate Note-Taking Companion

**Introduction:**

Memoir is a powerful and user-friendly note-taking tool designed to simplify how you capture, organize, and share your thoughts. Built using Rust, Memoir combines the speed and safety of the Rust programming language with the flexibility of a modern web application. It uses Axum for the backend, SQLx for database management, and can be tested seamlessly using Postman. Whether you're jotting down quick ideas, keeping detailed records, or sharing notes with others, Memoir offers a seamless experience that enhances productivity and collaboration.

**How It Works:**

Memoir allows users to create, manage, and share notes effortlessly. The application provides a RESTful API through Axum, where users can easily create new notes, retrieve them, or export them into text files. Each note is stored securely in an SQLite database managed by SQLx, ensuring your data is safe and easily retrievable. 

Memoir's API can be tested and interacted with using Postman, allowing you to send requests to the server to create, retrieve, update, or delete notes. You can also export your notes into `.txt` files with a simple API call, making it easy to share your thoughts or keep offline backups.

**Why Adopt Memoir?**

1. **Speed and Reliability:**  
   Built with Rust, Memoir offers blazing-fast performance and reliability. Rust’s safety features ensure that your data is managed efficiently, reducing the chances of crashes or data loss.

2. **Easy API Interaction:**  
   With Postman, testing Memoir's API is straightforward, allowing developers to interact with the application without needing a dedicated frontend.

3. **Secure and Private:**  
   Your notes are stored locally in an SQLite database, ensuring that your data remains private and secure.

4. **Versatile Export Options:**  
   The ability to convert notes into text files adds a layer of versatility to your note management, making it easier to share and archive your thoughts.

**Installation and Setup Instructions:**

Follow these steps to build, install, and run Memoir on your machine:

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/yourusername/memoir.git
   cd memoir
   ```

2. **Set Up the Environment:**
   Ensure you have Rust and SQLite installed on your machine. You can install Rust from [rust-lang.org](https://www.rust-lang.org/) and SQLite from [sqlite.org](https://sqlite.org/).

3. **Build the Backend:**
   Navigate to the `backend` directory and build the Rust application:
   ```bash
   cd backend
   cargo build --release
   ```

4. **Run the Application:**
   Start the backend server:
   ```bash
   cargo run --release
   ```

5. **Test with Postman:**
   - Install Postman from [postman.com](https://www.postman.com/).
   - Open Postman and create a new collection to manage your Memoir API requests.
   - Use Postman to send requests to `http://localhost:3000` (or the appropriate URL where your backend is running) to interact with the Memoir API. You can create, retrieve, update, delete, and export notes using various HTTP methods (GET, POST, PATCH, DELETE).

**Conclusion:**

Memoir is more than just a note-taking app—it's a robust tool that empowers users to manage their notes efficiently while providing powerful features like note sharing and export. With Memoir, you have a secure, fast, and easy-to-use platform that enhances your productivity. Give it a try, and experience a new level of note-taking!
