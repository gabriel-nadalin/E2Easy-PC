import matplotlib.pyplot as plt
import subprocess
import csv
from pathlib import Path

# Hardcoded N values to benchmark
N = [100, 200, 300, 400, 500]
REPS = 100  # number of executions per N to average

# Paths
CRATE = Path(__file__).resolve().parents[1] / 'mixnet-rust'  # /home/gabriel/tcc/mixnet-rust
BIN = CRATE / 'target' / 'release' / 'main'
CSV_OUT = Path(__file__).resolve().parents[1] / 'times_raw.csv'
TIMES_FIG = Path(__file__).resolve().parents[1] / 'times.jpeg'
CYCLES_FIG = Path(__file__).resolve().parents[1] / 'cycles.jpeg'

# Ensure binary exists (build once)
subprocess.run(['cargo', 'build', '--release'], cwd=str(CRATE), check=True)

def to_seconds(s: str) -> float:
    s = s.strip()
    if s.endswith('s') and not s.endswith('ms'):
        return float(s[:-1])
    if s.endswith('ms'):
        return float(s[:-2]) / 1_000.0
    if s.endswith('us') or s.endswith('µs'):
        return float(s[:-2]) / 1_000_000.0
    if s.endswith('ns'):
        return float(s[:-2]) / 1_000_000_000.0
    return float(s)

s_time = []
v_time = []
c_time = []
mix_cycles_avg = []
ver_cycles_avg = []
commit_cycles_avg = []

# Open CSV for raw data
with open(CSV_OUT, 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['N', 'Rep', 'Mixing_Time', 'Verifying_Time', 'Commits_Verifying_Time', 'Mixing_Cycles', 'Verifying_Cycles', 'Commits_Verifying_Cycles'])
    
    for n in N:
        mix_vals = []
        ver_vals = []
        com_vals = []
        mix_cycles_vals = []
        ver_cycles_vals = []
        commit_cycles_vals = []
        for rep in range(REPS):
            proc = subprocess.run([str(BIN), str(n)], cwd=str(CRATE), capture_output=True, text=True)
            out = proc.stdout.splitlines()
            mix_line      = next((l for l in out if l.startswith('Mixing time:')), None)
            mix_cycles_ln = next((l for l in out if l.startswith('Mixing cycles:')), None)
            ver_line      = next((l for l in out if l.startswith('Verifying time:')), None)
            ver_cycles_ln = next((l for l in out if l.startswith('Verifying cycles:')), None)
            com_line      = next((l for l in out if l.startswith('Commits verifying time:')), None)
            com_cycles_ln = next((l for l in out if l.startswith('Commits verifying cycles:')), None)
            if None in (mix_line, mix_cycles_ln, ver_line, ver_cycles_ln, com_line, com_cycles_ln):
                raise RuntimeError(f"Unexpected output for N={n}:\n" + proc.stdout)
            mix_sec   = to_seconds(mix_line.split(': ', 1)[1])
            ver_sec   = to_seconds(ver_line.split(': ', 1)[1])
            com_sec   = to_seconds(com_line.split(': ', 1)[1])
            mix_cycles = int(mix_cycles_ln.split(': ', 1)[1])
            ver_cycles = int(ver_cycles_ln.split(': ', 1)[1])
            commit_cycles = int(com_cycles_ln.split(': ', 1)[1])

            mix_vals.append(mix_sec)
            ver_vals.append(ver_sec)
            com_vals.append(com_sec)
            mix_cycles_vals.append(mix_cycles)
            ver_cycles_vals.append(ver_cycles)
            commit_cycles_vals.append(commit_cycles)

            writer.writerow([
                n,
                rep + 1,
                mix_line.split(': ', 1)[1],
                ver_line.split(': ', 1)[1],
                com_line.split(': ', 1)[1],
                mix_cycles,
                ver_cycles,
                commit_cycles
            ])

        s_time.append(sum(mix_vals) / len(mix_vals))
        v_time.append(sum(ver_vals) / len(ver_vals))
        c_time.append(sum(com_vals) / len(com_vals))
        mix_cycles_avg.append(sum(mix_cycles_vals) / len(mix_cycles_vals))
        ver_cycles_avg.append(sum(ver_cycles_vals) / len(ver_cycles_vals))
        commit_cycles_avg.append(sum(commit_cycles_vals) / len(commit_cycles_vals))

print(f"Raw timing data saved to: {CSV_OUT}")

plt.figure(figsize=(10, 6))
plt.plot(N, s_time, marker='o', label='Mixing Time')
plt.plot(N, v_time, marker='D', label='Verifying Time')
plt.plot(N, c_time, marker='s', label='Commits Verifying Time')
plt.title('Average Execution Time x Voters')
plt.xlabel('Voters')
plt.ylabel('Time (s)')
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig(TIMES_FIG, format='jpeg', dpi=300)

plt.figure(figsize=(10, 6))
scale = 1e9
plt.plot(N, [v/scale for v in mix_cycles_avg], marker='o', label='Mixing Cycles')
plt.plot(N, [v/scale for v in ver_cycles_avg], marker='D', label='Verifying Cycles')
plt.plot(N, [v/scale for v in commit_cycles_avg], marker='s', label='Commits Verifying Cycles')
plt.title('Average CPU Cycles x Voters')
plt.xlabel('Voters')
plt.ylabel('CPU Cycles (10⁹)')
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig(CYCLES_FIG, format='jpeg', dpi=300)

print(f"Time figure: {TIMES_FIG}")
print(f"Cycles figure: {CYCLES_FIG}")
