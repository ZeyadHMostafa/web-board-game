# train.py
import torch
import numpy as np
import csv
import os

from core_engine import PyTrainingEnvironment
from config import BATCH_CONFIGS, LR
from helpers import setup_training_session

def train():
    # Gather configuration from interactive prompt
    output_dir, delta_epochs, has_checkpoint = setup_training_session()

    # Define files relative to the chosen folder
    stats_file = os.path.join(output_dir, "training_statistics.csv")
    checkpoint_file = os.path.join(output_dir, "checkpoint_model.pt")
    partial_weights_file = os.path.join(output_dir, "trained_weights_partial.npy")
    final_weights_file = os.path.join(output_dir, "trained_weights.npy")

    # Initialize model architectures
    model = torch.nn.Linear(108, 1, bias=False)
    optimizer = torch.optim.Adam(model.parameters(), lr=LR)
    criterion = torch.nn.MSELoss()
    env = PyTrainingEnvironment()

    # Attempt to restore checkpoint progress
    start_epoch = 0
    if has_checkpoint:
        try:
            checkpoint = torch.load(checkpoint_file)
            model.load_state_dict(checkpoint["model_state"])
            optimizer.load_state_dict(checkpoint["optimizer_state"])
            start_epoch = checkpoint["epoch"] + 1
            print(f"\n-> Resuming safely from checkpoint at Epoch {start_epoch}...")
        except Exception as e:
            print(f"\n-> Failed to load checkpoint. Starting fresh. Info: {e}")

    # The user defined number of epochs to go *past* the current checkpoint
    total_target_epochs = start_epoch + delta_epochs
    print(f"-> Target epoch limit calculated: {total_target_epochs} (Training for {delta_epochs} more epochs)")

    # Initialize telemetry file headers
    file_exists = os.path.isfile(stats_file)
    with open(stats_file, mode="a", newline="") as f:
        writer = csv.writer(f)
        if not file_exists:
            writer.writerow(["Epoch", "Batch_Name", "Total_Samples", "Loss", "Weights_Norm"])

    print(f"\nBeginning optimization cycle inside target directory: '{output_dir}'")

    try:
        for epoch in range(start_epoch, total_target_epochs):
            print(f"\n--- Starting Epoch {epoch:03d} / {total_target_epochs - 1:03d} ---")
            
            # Run simulation batches for every custom-picked configuration inside this epoch
            for config in BATCH_CONFIGS:
                batch_name = config["name"]
                
                with torch.no_grad():
                    current_weights = model.weight.data.numpy().flatten().astype(np.float32)
                
                env.update_weights(current_weights)

                # Fetching configuration parameters dynamically
                np_features, np_targets = env.run_simulation(
                    config["num_games"],
                    config["search_depth"],
                    config["p1_mask"],
                    config["p2_mask"]
                )

                num_samples = np_features.shape[0]
                if num_samples == 0:
                    print(f" [{batch_name}] Generated 0 valid positions. Skipping.")
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

                # Record statistics per batch sub-step
                with open(stats_file, mode="a", newline="") as f:
                    writer = csv.writer(f)
                    writer.writerow([epoch, batch_name, num_samples, current_loss, weights_norm])

                print(f" [{batch_name:15s}] Samples: {num_samples:5d} | Loss: {current_loss:.6f} | Norm: {weights_norm:.4f}")

            # Intermediate state serialization after a full epoch sequence completes
            torch.save({
                "epoch": epoch,
                "model_state": model.state_dict(),
                "optimizer_state": optimizer.state_dict(),
            }, checkpoint_file)
            
            np.save(partial_weights_file, model.weight.data.numpy().flatten())

    except KeyboardInterrupt:
        print(f"\nTraining session interrupted manually. Progress cached cleanly inside '{output_dir}'.")

    # Final Export
    final_weights = model.weight.data.numpy().flatten()
    np.save(final_weights_file, final_weights)
    print(f"\nFinalized weights exported to {final_weights_file}.")

if __name__ == "__main__":
    train()