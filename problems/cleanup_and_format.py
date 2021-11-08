import os

if __name__ == "__main__":
    dirs = [d for d in os.listdir(".") if os.path.isdir(d)]
    allow_dirty = False
    for d in dirs:
        os.chdir(d)
        print(d)
        if allow_dirty:
            os.system("cargo clippy --fix --allow-dirty")
        os.system("cargo fmt")
        os.system("cargo clean")
        os.chdir("..")
    
    if allow_dirty:
        os.system("git branch dirty")
        os.system("git checkout dirty")

    os.system("git commit . -m " + "\"" + "formatting" + "\"")
    os.system("git push")
