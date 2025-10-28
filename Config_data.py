import json
from pathlib import Path

def create_json_config(file_path: str):
    # The configuration data structure
    config_data = {
        "service_name": "DataProcessor",
        "version": "1.2.0",
        "settings": {
            "max_workers": 8,
            "timeout_seconds": 30,
            "debug_mode": True
        },
        "database": {
            "host": "localhost",
            "port": 5432,
            "user": "admin"
        }
    }

    # 2. Write the dictionary to a JSON file
    try:
        # Use Path for modern file handling
        output_file = Path(file_path)
        
        with open(output_file, 'w') as f:
            # The 'indent=4' makes the file human-readable (pretty-printed)
            json.dump(config_data, f, indent=4)
        
        print(f"✅ Configuration file created successfully at: {output_file}")
        
    except IOError as e:
        print(f"❌ Error writing configuration file: {e}")
