import torch
import numpy as np
import csv
import os
from core_engine import PyTrainingEnvironment

def train(output_dir="training_run_01"):
    # Ensure the target directory structure exists
    os.makedirs(output_dir, exist_ok=True)

    # Resolve all file trajectories relative to the specified output folder
    stats_file = os.path.join(output_dir, "training_statistics.csv")
    checkpoint_file = os.path.join(output_dir, "checkpoint_model.pt")
    partial_weights_file = os.path.join(output_dir, "trained_weights_partial.npy")
    final_weights_file = os.path.join(output_dir, "trained_weights.npy")

    NUM_GAMES_PER_BATCH = 100
    SEARCH_DEPTH = 5
    EPOCHS = 320
    
    P1_START_MASK = 0x000000000000FFFF
    P2_START_MASK = 0xFFFF000000000000
    # P1_START_MASK = 0x000000003C3C3C3C
    # P2_START_MASK = 0x3C3C3C3C00000000

    model = torch.nn.Linear(108, 1, bias=False)
    optimizer = torch.optim.Adam(model.parameters(), lr=0.05)
    criterion = torch.nn.MSELoss()
    env = PyTrainingEnvironment()

    # Initialize telemetry file headers within the targeted folder
    file_exists = os.path.isfile(stats_file)
    with open(stats_file, mode="a", newline="") as f:
        writer = csv.writer(f)
        if not file_exists:
            writer.writerow(["Epoch", "Total_Samples", "Loss", "Weights_Norm"])

    # Attempt to restore progress if an interrupted run left a checkpoint in this folder
    start_epoch = 0
    if os.path.exists(checkpoint_file):
        try:
            checkpoint = torch.load(checkpoint_file)
            model.load_state_dict(checkpoint["model_state"])
            optimizer.load_state_dict(checkpoint["optimizer_state"])
            start_epoch = checkpoint["epoch"] + 1
            print(f"Resuming safely from checkpoint inside '{output_dir}' at Epoch {start_epoch}...")
        except Exception as e:
            print(f"Failed to load checkpoint from '{output_dir}', starting fresh. Info: {e}")

    print(f"Beginning model optimization cycle. Target directory: '{output_dir}'")

    try:
        for epoch in range(start_epoch, EPOCHS):
            with torch.no_grad():
                current_weights = model.weight.data.numpy().flatten().astype(np.float32)
            
            env.update_weights(current_weights)

            np_features, np_targets = env.run_simulation(
                NUM_GAMES_PER_BATCH,
                SEARCH_DEPTH,
                P1_START_MASK,
                P2_START_MASK
            )

            num_samples = np_features.shape[0]
            if num_samples == 0:
                print(f"Epoch {epoch}: Simulation generated 0 valid positions. Skipping.")
                continue

            features_tensor = torch.from_numpy(np_features)
            targets_tensor = torch.from_numpy(np_targets).unsqueeze(1)

            optimizer.zero_grad()
            predictions = model(features_tensor)
            loss = criterion(predictions, targets_tensor)
            loss.backward()
            optimizer.step()

            current_loss = loss.item()
            weights_norm = torch.norm(model.weight).item()

            # Record telemetry data persistently
            with open(stats_file, mode="a", newline="") as f:
                writer = csv.writer(f)
                writer.writerow([epoch, num_samples, current_loss, weights_norm])

            print(f"Epoch {epoch:03d} | Total Extracted Samples: {num_samples:5d} | Loss: {current_loss:.6f}")

            # Intermediate state serialization within the target folder
            torch.save({
                "epoch": epoch,
                "model_state": model.state_dict(),
                "optimizer_state": optimizer.state_dict(),
            }, checkpoint_file)
            
            # Export partial weights
            np.save(partial_weights_file, model.weight.data.numpy().flatten())

    except KeyboardInterrupt:
        print(f"\nTraining session interrupted manually. Progress cached cleanly inside '{output_dir}'.")

    # Final Export to target directory
    final_weights = model.weight.data.numpy().flatten()
    np.save(final_weights_file, final_weights)
    print(f"Finalized weights exported to {final_weights_file}.")

if __name__ == "__main__":
    # Specify the target destination directory name here
    train(output_dir="data/exp1")