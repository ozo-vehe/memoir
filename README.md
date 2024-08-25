## Memoir: Your Ultimate Note-Taking Companion

**Introduction:**

Memoir is a powerful and user-friendly note-taking tool designed to simplify how you capture, organize, and share your thoughts. Built using Rust, Memoir combines the speed and safety of the Rust programming language with the flexibility of a modern web application, using Axum for the backend, SQLx for database management, and Vue for the frontend. Whether you're jotting down quick ideas, keeping detailed records, or sharing notes with others, Memoir offers a seamless experience that enhances productivity and collaboration.

**How It Works:**

Memoir allows users to create, manage, and share notes effortlessly. The application provides a clean and intuitive interface through Vue, where users can easily create new notes. Each note is stored securely in an SQLite database managed by SQLx, ensuring your data is safe and easily retrievable. The backend, powered by Axum, handles all the operations, including note creation, retrieval, and conversion to text files.

One of Memoir's standout features is its ability to convert notes into text files. With just a click, you can export your notes into `.txt` files, making it easy to share your thoughts with others or keep offline backups. Additionally, Memoir enables you to share notes directly within the application, facilitating collaboration among team members or friends.

**Why Adopt Memoir?**

1. **Speed and Reliability:**  
   Built with Rust, Memoir offers blazing-fast performance and reliability. Rust’s safety features ensure that your data is managed efficiently, reducing the chances of crashes or data loss.

2. **User-Friendly Interface:**  
   The Vue-based frontend provides a smooth and responsive user experience. Even if you're not tech-savvy, you'll find Memoir easy to navigate and use.

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
   Ensure you have Rust, Node.js, and SQLite installed on your machine. You can install Rust from [rust-lang.org](https://www.rust-lang.org/), Node.js from [nodejs.org](https://nodejs.org/), and SQLite from [sqlite.org](https://sqlite.org/).

3. **Build the Backend:**
   Navigate to the `backend` directory and build the Rust application:
   ```bash
   cd backend
   cargo build --release
   ```

4. **Set Up the Frontend:**
   Navigate to the `frontend` directory and install the dependencies:
   ```bash
   cd ../frontend
   npm install
   npm run build
   ```

5. **Run the Application:**
   Start the backend server:
   ```bash
   cd ../backend
   cargo run --release
   ```
   Open another terminal window and serve the frontend:
   ```bash
   cd ../frontend
   npm run serve
   ```

6. **Access Memoir:**
   Once the server is running, open your web browser and navigate to `http://localhost:3000` to start using Memoir.

**Conclusion:**

Memoir is more than just a note-taking app—it's a robust tool that empowers users to manage their notes efficiently while providing powerful features like note sharing and export. With Memoir, you have a secure, fast, and easy-to-use platform that enhances your productivity. Give it a try, and experience a new level of note-taking!
