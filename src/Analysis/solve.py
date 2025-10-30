import matplotlib.pyplot as plt

N = []
s1_time = []
v1_time = []
with open('times', 'r') as file:
    i = 0
    for line in file:
        if i % 3 == 0:
           N.append(int(line.split(' = ')[1])) 
        elif i % 3 == 1:
            s1_time.append(float(line.split(': ')[1])) 
        else:
            v1_time.append(float(line.split(': ')[1])) 
        i += 1
s2_time = []
v2_time = []
cv_time = []
with open('times2', 'r') as file:
    i = 0
    for line in file:
        if i % 4 == 0:
            i += 1
            continue
            # N.append(int(line.split(' = ')[1])) 
        elif i % 4 == 1:
            s2_time.append(float(line.split(': ')[1])) 
        elif i % 4 == 2:
            v2_time.append(float(line.split(': ')[1])) 
        else:
            cv_time.append(float(line.split(': ')[1])) 
        i += 1
plt.plot(N, s1_time, marker='s', color='#130ee2', label='El Gamal Shuffler Time')
plt.plot(N, v1_time, marker='D', color='#13d200', label='El Gamal Verifier Time')
plt.plot(N, s2_time, marker='x', color='#0e80e2', label='Pedersen Shuffler Time')
plt.plot(N, v2_time, marker='+', color='#00d28e', label='Pedersen Verifier Time')
#plt.plot(N, cv_time, marker='+', label='Commitments Verification Time')
plt.title('Mixnet Execution Time x N')
plt.xlabel('N')
plt.ylabel('Time (s)')
plt.legend()
plt.grid(True)
plt.show()
#plt.savefig('times.jpeg', format='jpeg', dpi=300)
