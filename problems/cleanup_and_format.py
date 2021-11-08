import os

if __name__ == "__main__":
    dirs = [d for d in os.listdir(".") if os.path.isdir(d)]
    for d in dirs:
        os.chdir(d)
        print(d)
        os.system("cargo clippy --fix")
        os.system("cargo fmt")
        os.system("cargo clean")
        os.chdir("..")
