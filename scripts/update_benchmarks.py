import subprocess
import re
import os
import sys

def run_benchmarks():
    print("Running benchmarks...")
    # Run cargo bench and capture output
    # We use --color never to make parsing easier
    result = subprocess.run(
        ["cargo", "bench", "--", "--color", "never"], 
        capture_output=True, 
        text=True
    )
    
    if result.returncode != 0:
        print("Benchmark failed:")
        print(result.stderr)
        sys.exit(1)
        
    return result.stdout

def parse_results(output):
    # Regex to capture: "Group/Scenario/Library time: [min mid max] unit"
    # Example: "Decoding/Uint256/zabi-rs time:   [2.3364 ns 2.3421 ns 2.3486 ns]"
    # We want the middle value and the unit.
    
    # Pattern: ^Group/Scenario/Lib ... time: ... [ ... mid ... ]
    # But criterion output is multiline sometimes or has other noise.
    # Lines look like:
    # Decoding/Uint256/zabi-rs time:   [2.3364 ns 2.3421 ns 2.3486 ns]
    
    regex = r"([\w\/]+)\s+time:\s+\[[\d\.]+\s+\w+\s+([\d\.]+)\s+(\w+)\s+[\d\.]+\s+\w+\]"
    
    data = {}
    
    for line in output.splitlines():
        match = re.search(regex, line)
        if match:
            full_name = match.group(1) # e.g. Decoding/Uint256/zabi-rs
            time_val = match.group(2)
            time_unit = match.group(3)
            
            # Parse full_name
            # Expected format: Decoding/Scenario/Library
            parts = full_name.split('/')
            if len(parts) >= 3:
                scenario = parts[1]
                library = parts[2]
                
                if scenario not in data:
                    data[scenario] = {}
                
                data[scenario][library] = f"{time_val} {time_unit}"
    
    return data

def generate_table(data):
    if not data:
        return "No benchmark data found."
        
    # Collect all libraries found
    libraries = set()
    for scenario in data:
        for lib in data[scenario]:
            libraries.add(lib)
    
    # Sort libraries (zabi-rs first, then others)
    libs_sorted = sorted(list(libraries))
    if 'zabi-rs' in libs_sorted:
        libs_sorted.remove('zabi-rs')
        libs_sorted.insert(0, 'zabi-rs')
        
    # Header
    table = "| Scenario | " + " | ".join(libs_sorted) + " |\n"
    table += "|----------|" + "|".join(["---" for _ in libs_sorted]) + "|\n"
    
    # Rows
    for scenario in sorted(data.keys()):
        row = f"| {scenario} | "
        for lib in libs_sorted:
            val = data[scenario].get(lib, "N/A")
            row += f"{val} | "
        table += row + "\n"
        
    return table

def update_readme(table):
    readme_path = "README.md"
    with open(readme_path, "r") as f:
        content = f.read()
    
    start_marker = "<!-- BENCHMARK_TABLE_START -->"
    end_marker = "<!-- BENCHMARK_TABLE_END -->"
    
    if start_marker not in content or end_marker not in content:
        print("Markers not found in README.md")
        return
    
    # Replace content
    pattern = re.compile(f"{re.escape(start_marker)}.*?{re.escape(end_marker)}", re.DOTALL)
    new_content = pattern.sub(f"{start_marker}\n\n{table}\n{end_marker}", content)
    
    with open(readme_path, "w") as f:
        f.write(new_content)
    
    print("README.md updated.")

if __name__ == "__main__":
    output = run_benchmarks()
    # For debugging, print a bit of output
    # print(output[:500])
    
    data = parse_results(output)
    print("Parsed data:", data)
    
    table = generate_table(data)
    print("Generated Table:\n", table)
    
    update_readme(table)
