# %%
import json
import numpy as np
import matplotlib.pyplot as plt
import os


def main():
    json_dir = './simulation_output_json'
    if not os.path.exists(json_dir):
        raise FileNotFoundError(
            'There is no directory with json results for axelrod simulation'
        )
    out_dir = './simulation_culture_map_images'
    if not os.path.exists(out_dir):
        os.mkdir(out_dir)
    
    simulation_files = [
        os.path.join(json_dir, js)
        for js in os.listdir(json_dir) if "json" in js
        ]
    for sf in simulation_files:
        with open(sf, 'r') as f:
            sim = json.load(f)
        territory = sim['territory']
        size = len(territory)

        culture_labels = np.zeros((size, size))

        for i, j in ((i, j) for i in range(size) for j in range(size)):
            culture_labels[i, j] = territory[i][j]['culture_label']

        sim_config = {}        
        name = sf.split(".")[1].split("_")
        for n in name:
            if "-" in n:
                key, val = n.split("-")
                sim_config[key] = val
        
        fig, ax = plt.subplots(1, 1)
        fig.suptitle("Simulation Terrain")
        ax.set(
            title=f"Size: {sim_config['size']}\t"\
                f"Features: {sim_config['features']}\t"\
                f"Traits: {sim_config['traits']}"
        )
        ax.imshow(culture_labels, cmap=plt.get_cmap("tab20b"))
        out_name = f"./{out_dir}/sim_image_size-{sim_config['size']}_"\
                f"features-{sim_config['features']}_"\
                f"traits-{sim_config['traits']}.png"
        fig.savefig(out_name)
        plt.close()


if __name__ == '__main__':
    main()
