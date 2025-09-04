La Liga Fantasy Data Scraper
A concurrent Rust CLI for scraping and compiling La Liga fantasy football player rosters.

📖 About The Project
As a manager in a fantasy La Liga, staying ahead requires quick access to accurate and comprehensive player data. Manually checking websites for rosters, player stats, and prices is time-consuming and inefficient.

This project aims to solve that problem by providing a powerful, fast, and reliable command-line tool to automate the collection of this data. Built entirely in Rust, it leverages asynchronous programming to fetch information for all 20 La Liga teams concurrently, ensuring high performance and efficiency.

The ultimate goal is to evolve this tool into a comprehensive data source for any analytical fantasy football project.

✨ Features
Concurrent Scraping: Fetches data for all La Liga teams simultaneously, making the process incredibly fast.

Roster Compilation: Scrapes the basic player roster for every team in the league.

Formatted Output: Saves the collected data into a clean, human-readable .txt file.

Built for Performance: Written in Rust for maximum speed and memory safety.

🚀 Getting Started
To get a local copy up and running, follow these simple steps.

Prerequisites
You must have the Rust programming language and its package manager, Cargo, installed on your system. You can install it from rust-lang.org.

Installation
Clone the repository to your local machine:

Bash

git clone https://github.com/your_username/your_repository.git
Navigate into the project directory:

Bash

cd your_repository
Build the project in release mode for optimal performance:

Bash

cargo build --release
💻 Usage
Once the project is built, you can run the scraper from your terminal:

Bash

cargo run --release
The application will begin fetching the data, printing its progress to the console. Upon completion, you will find a new file named laliga_players.txt in the project's root directory containing the player rosters.

🛠️ Methodology: How It Works
This project's scraping methodology is designed to be robust and adaptable. The core challenge is not just fetching HTML, but accurately parsing it to extract specific, meaningful data points.

The current approach involves two main strategies:

Simple Roster Scraping: The initial version of the script targets the main team roster pages. It uses a simple but effective CSS selector (a.jugador) to identify and extract the name of every player on the team, compiling a basic roster list.

Advanced Statistical Scraping (The Core Logic): To extract detailed statistics (like price, points, position, etc.), a more sophisticated pattern is required. The key insight is that most statistical data points are presented in a label-value pair. For example:

HTML

<div class="label">Partidos jugados</div>
<div class="value">3</div>
The logic to capture this accurately is to:

Find the Landmark: First, locate the "label" element by its specific text (e.g., "Partidos jugados").

Select the Sibling: Once the label is found, programmatically navigate to its immediate HTML sibling, which contains the corresponding "value".

Extract the Data: Extract the text from the "value" element.

This "landmark and sibling" approach is far more reliable than guessing selectors and ensures that the data being extracted is correctly associated with its description.

🗺️ Project Goals & Roadmap
The current version provides a solid foundation, but the goal is to build a much more powerful data-gathering tool.

[ ] Full Statistical Extraction: Expand the scraper to visit each player's individual page and apply the "landmark and sibling" logic to extract a complete set of statistics:

position

price

avg_points

goals & assists

cards & minutes played

...and all other relevant fantasy points data.

[ ] Structured Data Output: Transition from a .txt file to a more useful, structured format like JSON or CSV. This will make the output data easily consumable by other applications, scripts, or data analysis tools.

[ ] Data Storage: Integrate a simple database (like SQLite) to store the data, allowing for historical tracking and more complex queries.

[ ] Advanced Configuration: Allow users to specify which teams or stats to scrape via command-line arguments.

📜 License
Distributed under the MIT License. See LICENSE for more information.
