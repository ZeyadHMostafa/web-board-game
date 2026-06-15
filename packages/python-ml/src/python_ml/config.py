BATCH_CONFIGS = [
    {
        "name": "center_dense",
        "p1_mask": 0x0000_0000_3C3C_3C3C,
        "p2_mask": 0x3C3C_3C3C_0000_0000,
        "num_games": 150,
        "search_depth": 4,
    },
    {
        "name": "basin",
        "p1_mask": 0x0000_0000_003C_7E7E,
        "p2_mask": 0x7E7E_3C00_0000_0000,
        "num_games": 90,
        "search_depth": 4,
    },
    {
        "name": "circle",
        "p1_mask": 0x0000_0000_7E7E_3C00,
        "p2_mask": 0x003C_7E7E_0000_0000,
        "num_games": 90,
        "search_depth": 4,
    },
    {
        "name": "row_split",
        "p1_mask": 0x0000_0000_0000_FFFF,
        "p2_mask": 0xFFFF_0000_0000_0000,
        "num_games": 60,
        "search_depth": 4,
    }
]

LR = 0.05