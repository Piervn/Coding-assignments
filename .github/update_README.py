import os

def get_folder_names(path):
    return [name for name in os.listdir(path) if os.path.isdir(os.path.join(path, name))]

def get_files(path):
    return [name for name in os.listdir(path) if os.path.isfile(os.path.join(path, name))]

print(os.getcwd())
main_path = os.getcwd()
all_folders = get_folder_names(main_path)
excluded_folders = ['.github', '.git']
folders = list(set(all_folders) - set(excluded_folders))

dict = {}
for lang in folders:
    dict[lang] = {}
    sources = get_folder_names(os.path.join(main_path, lang))
    for source in sources:
        number_of_assignments = 0
        path = os.path.join(main_path, lang, source)
        match lang:
            case "Rust":
                path = os.path.join(path, "src", "bin")
        files = get_files(path)
        dict[lang][source] = len(files)

print(dict)
    