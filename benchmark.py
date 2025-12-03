import matplotlib.pyplot as plt
import subprocess
import csv
from pathlib import Path

# Hardcoded N values to benchmark
N = [100, 200, 300, 400, 500]
REPS = 10  # number of executions per N to average

# Paths
CRATE = Path(__file__).resolve().parents[1] / 'mixnet-rust'  # /home/gabriel/tcc/mixnet-rust
BIN = CRATE / 'target' / 'release' / 'main'
CSV_OUT = Path(__file__).resolve().parents[1] / 'cycles_raw.csv'
CYCLES_FIG = Path(__file__).resolve().parents[1] / 'cycles.pdf'

# Ensure binary exists (build once)
subprocess.run(['cargo', 'build', '--release'], cwd=str(CRATE), check=True)

vote_cycles_avg = []
cast_cycles_avg = []
mix_cycles_avg = []
ver_cycles_avg = []
commit_cycles_avg = []

# Open CSV for raw data
with open(CSV_OUT, 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['N', 'Rep', 'Voting_Cycles', 'Casting_Cycles', 'Mixing_Cycles', 'Verifying_Cycles', 'Commits_Verifying_Cycles'])
    
    for n in N:
        vote_cycles_vals = []
        cast_cycles_vals = []
        mix_cycles_vals = []
        ver_cycles_vals = []
        commit_cycles_vals = []
        for rep in range(REPS):
            proc = subprocess.run([str(BIN), str(n)], cwd=str(CRATE), capture_output=True, text=True)
            out = proc.stdout.splitlines()
            vote_cycles_ln = next((l for l in out if l.startswith('Voting cycles:')), None)
            cast_cycles_ln = next((l for l in out if l.startswith('Casting cycles:')), None)
            mix_cycles_ln = next((l for l in out if l.startswith('Mixing cycles:')), None)
            ver_cycles_ln = next((l for l in out if l.startswith('Verifying cycles:')), None)
            com_cycles_ln = next((l for l in out if l.startswith('Commits verifying cycles:')), None)
            if None in (vote_cycles_ln, cast_cycles_ln, mix_cycles_ln, ver_cycles_ln, com_cycles_ln):
                raise RuntimeError(f"Unexpected output for N={n}:\n" + proc.stdout)
            vote_cycles = int(vote_cycles_ln.split(': ', 1)[1])
            cast_cycles = int(cast_cycles_ln.split(': ', 1)[1])
            mix_cycles = int(mix_cycles_ln.split(': ', 1)[1])
            ver_cycles = int(ver_cycles_ln.split(': ', 1)[1])
            commit_cycles = int(com_cycles_ln.split(': ', 1)[1])

            vote_cycles_vals.append(vote_cycles)
            cast_cycles_vals.append(cast_cycles)
            mix_cycles_vals.append(mix_cycles)
            ver_cycles_vals.append(ver_cycles)
            commit_cycles_vals.append(commit_cycles)

            writer.writerow([
                n,
                rep + 1,
                vote_cycles,
                cast_cycles,
                mix_cycles,
                ver_cycles,
                commit_cycles
            ])

        vote_cycles_avg.append(sum(vote_cycles_vals) / len(vote_cycles_vals))
        cast_cycles_avg.append(sum(cast_cycles_vals) / len(cast_cycles_vals))
        mix_cycles_avg.append(sum(mix_cycles_vals) / len(mix_cycles_vals))
        ver_cycles_avg.append(sum(ver_cycles_vals) / len(ver_cycles_vals))
        commit_cycles_avg.append(sum(commit_cycles_vals) / len(commit_cycles_vals))

print(f"Raw cycles data saved to: {CSV_OUT}")

plt.figure(figsize=(10, 6))
scale = 1e9
plt.plot(N, [v/scale for v in vote_cycles_avg], marker='o', label='Voting Cycles')
plt.plot(N, [v/scale for v in cast_cycles_avg], marker='s', label='Casting Cycles')
plt.plot(N, [v/scale for v in mix_cycles_avg], marker='^', label='Mixing Cycles')
plt.plot(N, [v/scale for v in ver_cycles_avg], marker='D', label='Verifying Cycles')
plt.plot(N, [v/scale for v in commit_cycles_avg], marker='*', label='Commits Verifying Cycles')
plt.title('Average CPU Cycles x Voters')
plt.xlabel('Voters')
plt.ylabel('CPU Cycles (10⁹)')
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig(CYCLES_FIG, format='pdf')

print(f"Cycles figure: {CYCLES_FIG}")
