## brainstorming for language features

+ entry point is main()
+ Memory Management: destructors of values run after leaving an scope / { }-Block
+ String,int(i64) float(f64) and bool as data types
+ Options as null replacement
+ build-in functions 

| Function        | behavior                                                            |
| -------------   |:-------------:                                                      |
| register        | register an function for an hotkey                                  |
| spawn_proc      | starts an process and dont wait for termination, returns pid as int |
| spawn_proc_wait | spawns a process and waits for termination                          |
| backup          | backups an file or directory to the given path                      |
| print           | prints string on stdout                                             |
| kill            | kills process with given pid                                        |

- finding process by name?