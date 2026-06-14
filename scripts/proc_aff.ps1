$procs = Get-Process -Name cargo,rustc -ErrorAction SilentlyContinue
foreach ($p in $procs) {
    "{0,-7} {1,-6} aff=0x{2:X} threads={3} cpu={4}s" -f $p.Id, $p.Name, [int64]$p.ProcessorAffinity, $p.Threads.Count, [int]$p.CPU
}
"---count---"
($procs | Measure-Object).Count
