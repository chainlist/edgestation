/** Network throughput in bytes per second. */
export interface NetRate {
	down: number;
	up: number;
}

/**
 * The performance of the machine at the time of the call. The host measures the CPU load and
 * network speeds since the previous call from any widget. Calls less than 500 ms apart share
 * one reading.
 */
export interface SystemInfo {
	hostname: string;
	/**
	 * The name of the connected Wi-Fi network: its Windows profile name, or the name of its
	 * NetworkManager connection on Linux. `null` if none.
	 */
	wifi: string | null;
	/** The load of all CPU cores, in percent. */
	cpu: { load: number };
	/** The first NVIDIA GPU (load in percent, temperature in °C). `null` if there is none. */
	gpu: { load: number; temperature: number } | null;
	/**
	 * The rate since the previous call, and the averages over the calls of the last 1, 5 and 15
	 * minutes. An average covers its full period while widgets continue to ask. It covers less
	 * time just after they start or resume.
	 */
	net: NetRate & { avg1m: NetRate; avg5m: NetRate; avg15m: NetRate };
}

/** The CPU use of a program: the sum of all processes that show the same name. */
export interface ProgramUsage {
	/**
	 * The name that Task Manager shows. This is the description of its executable (`THE FINALS`,
	 * `Google Chrome`), else the executable file name without `.exe` (`crashpad_handler`). On
	 * Linux, it is the name of its app in the app menu, else the executable file name.
	 */
	name: string;
	/** The percent of all logical processors, as in Task Manager. */
	cpu: number;
}
