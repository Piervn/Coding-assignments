import os
from jinja2 import Environment, FileSystemLoader

def get_folder_names(path):
    return [name for name in os.listdir(path) if os.path.isdir(os.path.join(path, name))]

def get_files(path):
    return [name for name in os.listdir(path) if os.path.isfile(os.path.join(path, name))]

# All languages
print(os.getcwd())
main_path = os.getcwd()
all_folders = get_folder_names(main_path)
excluded_folders = ['.github', '.git', '.vscode']
folders = list(set(all_folders) - set(excluded_folders))

# languages + subjects + totals
assignments = {}
columns = set()
total_rows = {}
total_columns = {}
for lang in folders:
    assignments[lang] = {}
    if lang not in total_rows.keys():
        total_rows[lang] = 0
    subjects = list(set(get_folder_names(os.path.join(main_path, lang))) - set(excluded_folders))
    for subject in subjects:
        if subject not in total_columns.keys():
            total_columns[subject] = 0
        else:
            total_columns[subject] += 1
        columns.add(subject)
        number_of_assignments = 0
        path = os.path.join(main_path, lang, subject)
        match lang:
            case "Rust":
                path = os.path.join(path, "src", "bin")
        files = get_files(path)
        assignments[lang][subject] = len(files)
        total_rows[lang] += len(files)


for platform in columns:
    total_columns[platform] = 0
    for lang in assignments.keys():
        if platform in assignments[lang].keys():
            total_columns[platform] += assignments[lang][platform]

# prepare data
columns = sorted(list(columns))
total_rows = dict(sorted(total_rows.items(), key=lambda x: x[0]))
total_columns = map(lambda x: x[1], sorted(total_columns.items(), key=lambda x: x[0]))

for lang in assignments.keys():
    assignments[lang] = [
        assignments[lang][source] if source in assignments[lang].keys() else '-' 
        for source in columns
    ]

file_loader = FileSystemLoader('.github')
env = Environment(loader = file_loader)
template = env.get_template('template_README.md.jinja')
data = {
    'total_rows': total_rows,
    'total_columns': total_columns,
    'TOTAL': sum(total_rows.values()),
    'columns': columns,
    'assignments': assignments,
}
output = template.render(data)
with open('README.md', 'w') as f:
    f.write(output)
    