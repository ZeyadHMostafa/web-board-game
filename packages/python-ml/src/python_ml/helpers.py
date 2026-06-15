import os
import shutil

def setup_training_session():
    print("=== Training Configuration Setup ===")
    print("Press [Enter] at any prompt to choose the default (path of least resistance).")
    
    # 1. Base Sub-path Selection
    default_subpath = "data/exp1"
    user_path = input(f"\nEnter output sub-path [{default_subpath}]: ").strip()
    output_dir = user_path if user_path else default_subpath

    # Checkpoint check
    checkpoint_file = os.path.join(output_dir, "checkpoint_model.pt")
    has_checkpoint = os.path.exists(checkpoint_file)

    # 2. Overwrite vs. Continue Logic
    mode = "c"  # default is continue
    if os.path.exists(output_dir):
        if has_checkpoint:
            user_mode = input(f"Existing checkpoint found in '{output_dir}'. [C]ontinue or [O]verwrite? [C]: ").strip().lower()
            if user_mode == 'o':
                mode = 'o'
        else:
            user_mode = input(f"Directory '{output_dir}' exists but has no checkpoint. [C]ontinue/Merge or [O]verwrite? [C]: ").strip().lower()
            if user_mode == 'o':
                mode = 'o'
                
    if mode == 'o' and os.path.exists(output_dir):
        print(f"Overwriting directory: Removing old data in '{output_dir}'...")
        shutil.rmtree(output_dir)
        has_checkpoint = False

    os.makedirs(output_dir, exist_ok=True)

    # 3. Delta Epochs Logic
    default_delta_epochs = 5
    while True:
        user_epochs = input(f"Enter number of ADDITIONAL epochs to train [{default_delta_epochs}]: ").strip()
        if not user_epochs:
            delta_epochs = default_delta_epochs
            break
        try:
            delta_epochs = int(user_epochs)
            if delta_epochs > 0:
                break
            print("Please enter a positive integer.")
        except ValueError:
            print("Invalid input. Please enter a number.")

    return output_dir, delta_epochs, has_checkpoint