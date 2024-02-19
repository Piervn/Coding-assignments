import os
from jinja2 import Environment, FileSystemLoader

def get_folder_names(path):
    return [name for name in os.listdir(path) if os.path.isdir(os.path.join(path, name))]

def get_files(path):
    return [name for name in os.listdir(path) if os.path.isfile(os.path.join(path, name))]

print(os.getcwd())
main_path = os.getcwd()
all_folders = get_folder_names(main_path)
excluded_folders = ['.github', '.git', '.vscode']
folders = list(set(all_folders) - set(excluded_folders))

assignments = {}
for lang in folders:
    assignments[lang] = {}
    sources = list(set(get_folder_names(os.path.join(main_path, lang))) - set(excluded_folders))
    for source in sources:
        number_of_assignments = 0
        path = os.path.join(main_path, lang, source)
        match lang:
            case "Rust":
                path = os.path.join(path, "src", "bin")
        files = get_files(path)
        assignments[lang][source] = len(files)

total = {}
for lang in assignments.keys():
    total[lang] = sum(assignments[lang].values())
total = sorted(total.items(), key=lambda x: x[1], reverse=True)

platforms = set()
for lang in assignments.keys():
    for source in assignments[lang].keys():
        platforms.add(source)
        
platforms = sorted(list(platforms))
        
for lang in assignments.keys():
    assignments[lang] = [
        assignments[lang][source] if source in assignments[lang].keys() else '-' 
        for source in platforms
    ]

file_loader = FileSystemLoader('.github')
env = Environment(loader = file_loader)
template = env.get_template('template_README.md.jinja')
data = {
    'total': total,
    'platforms': ' | '.join(platforms),
    'platforms_count': len(platforms),
    'assignments': assignments,
}
output = template.render(data)
with open('README.md', 'w') as f:
    f.write(output)
    