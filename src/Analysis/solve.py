import matplotlib.pyplot as plt

N = []
s_time = []
v_time = []
with open('times', 'r') as file:
    i = 0
    for line in file:
        if i % 3 == 0:
           N.append(int(line.split(' = ')[1])) 
        elif i % 3 == 1:
            s_time.append(float(line.split(': ')[1])) 
        else:
            v_time.append(float(line.split(': ')[1])) 
        i += 1
plt.plot(N, s_time, marker='o', label='Shuffler Time')
plt.plot(N, v_time, marker='D', label='Verifier Time')
plt.title('Mixnet Execution Time x N')
plt.xlabel('N')
plt.ylabel('Time (s)')
plt.legend()
plt.grid(True)
plt.show()
plt.savefig('times.jpeg', format='jpeg', dpi=300)
