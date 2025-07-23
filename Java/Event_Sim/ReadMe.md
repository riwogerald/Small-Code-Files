# EveSim - Single-Server Queueing System Simulation

## Overview

EveSim is a discrete-event simulation program that models a single-server queueing system. This Java application simulates customer arrivals, service times, and queue behavior to analyze system performance metrics such as average waiting time, queue length, and server utilization.

## What is a Single-Server Queueing System?

A single-server queueing system is a fundamental model in operations research and computer science where:
- **Customers arrive** at random intervals
- **One server** provides service to customers
- **Queue forms** when the server is busy and new customers arrive
- **Service times** vary randomly
- **Performance metrics** help evaluate system efficiency

Common real-world examples include:
- Bank teller windows
- Customer service call centers
- Computer processor task scheduling
- Traffic light systems
- Medical clinic appointment systems

## Simulation Model

### Key Components

1. **Arrival Process**: Customers arrive according to an exponential distribution with a specified mean interarrival time
2. **Service Process**: Service times follow an exponential distribution with a specified mean service time
3. **Queue**: First-In-First-Out (FIFO) queue with a maximum capacity limit
4. **Server**: Single server that can be either busy or idle

### Events Simulated

- **Arrival Event**: A new customer arrives at the system
- **Departure Event**: A customer completes service and leaves the system

## Input Parameters

The simulation requires three input parameters in the `infile.txt` file:

```
<mean_interarrival_time> <mean_service_time> <number_of_customers>
```

### Example Input
```
1.0 0.5 100
```

- **Mean Interarrival Time**: 1.0 minutes (average time between customer arrivals)
- **Mean Service Time**: 0.5 minutes (average time to serve one customer)
- **Number of Customers**: 100 (total customers to simulate)

## Output Metrics

The simulation generates the following performance metrics:

1. **Average Delay in Queue**: Average waiting time for customers before service begins
2. **Average Number in Queue**: Time-weighted average number of customers waiting
3. **Server Utilization**: Proportion of time the server is busy
4. **Simulation End Time**: Total time elapsed during the simulation

### Example Output
```
SINGLE-SERVER QUEUING SYSTEM

Mean interarrival time: 1.0 minutes
Mean service time: 0.5 minutes
Number of customers: 100

Average delay in queue: 0.53771 minutes
Average number in queue: 0.49738
Server utilization: 0.47076
Time simulation ended: 108.10713
```

## File Structure

```
EveSim/
├── EveSim.java          # Main simulation program
├── infile.txt           # Input parameters file
├── outfile.txt          # Generated simulation results
└── README.md            # This documentation
```

## How to Run

### Prerequisites
- Java Development Kit (JDK) 8 or higher
- Text editor for modifying input parameters

### Compilation and Execution

1. **Compile the program**:
   ```bash
   javac EveSim.java
   ```

2. **Prepare input file**: Create or modify `infile.txt` with your desired parameters:
   ```
   1.0 0.5 100
   ```

3. **Run the simulation**:
   ```bash
   java EveSim
   ```

4. **View results**: Check the generated `outfile.txt` for simulation results

## Understanding the Results

### Interpreting Metrics

- **Low Average Delay**: System is well-balanced, customers don't wait long
- **High Server Utilization**: Server is busy most of the time (good resource usage)
- **Average Number in Queue**: Indicates typical congestion level

### System Performance Analysis

- **Utilization < 1.0**: System is stable (arrival rate < service rate)
- **Utilization ≈ 1.0**: System approaching capacity limits
- **High queue length**: May indicate need for additional servers

## Technical Implementation

### Simulation Algorithm

1. **Event-Driven Approach**: Uses discrete-event simulation methodology
2. **Event List Management**: Maintains chronological order of future events
3. **Statistical Collection**: Accumulates time-weighted statistics during simulation
4. **Random Number Generation**: Uses exponential distribution for realistic modeling

### Key Classes and Methods

- `initialize()`: Sets up initial simulation state
- `timing()`: Determines next event and advances simulation clock  
- `arrive()`: Processes customer arrival events
- `depart()`: Processes customer departure events
- `update_time_avg_stats()`: Updates statistical accumulators
- `expon()`: Generates exponentially distributed random numbers

## Customization Options

### Modifying Parameters

You can experiment with different scenarios by changing the input parameters:

- **Increase interarrival time**: Reduces customer arrival rate
- **Decrease service time**: Improves service efficiency
- **Change customer count**: Affects simulation length and statistical accuracy

### Example Scenarios

1. **Busy System**: `0.5 1.0 100` (arrivals faster than service)
2. **Efficient System**: `2.0 0.5 100` (service much faster than arrivals)
3. **Balanced System**: `1.0 1.0 100` (arrival rate equals service rate)

## Limitations

- **Single Server Only**: Cannot model multiple-server systems
- **FIFO Queue**: No priority or other queue disciplines
- **Exponential Distributions**: Real systems may have different distributions
- **No Balking/Reneging**: Customers always join queue and wait for service

## Educational Value

This simulation is excellent for learning:
- Discrete-event simulation concepts
- Queueing theory fundamentals
- Statistical analysis of random systems
- Java programming with file I/O
- Performance metric interpretation

## Future Enhancements

Possible extensions to the simulation:
- Multiple server support
- Different arrival/service distributions
- Priority queue disciplines
- Customer balking and reneging
- Cost analysis features
- Graphical visualization

## References

This implementation is based on classical queueing theory and discrete-event simulation principles commonly taught in operations research and computer science courses.

## License

This project is provided for educational purposes. Feel free to modify and use for learning and research.