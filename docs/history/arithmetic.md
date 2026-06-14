# History — Arithmetic

How the arithmetic operations have moved release over release. See the
[History overview](../history.md) for the time units, the width reference map, and how
these timings are measured.

<!-- BEGIN GENERATED:history:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 220 ns (0.9×) | 215 ns (0.88×) | 245 ns |
| D38 | 331 ns (0.64×) | 351 ns (0.68×) | 520 ns |
| D57 | · | 2.21 µs (2.8×) | 796 ns |
| D76 | 1 µs (1×) | 2.62 µs (2.6×) | 992 ns |
| D115 | · | 5.24 µs (1.7×) | 3.02 µs |
| D153 | 1.7 µs (0.4×) | 7.93 µs (1.9×) | 4.21 µs |
| D230 | 3.08 µs (0.38×) | 11.8 µs (1.5×) | 8.04 µs |
| D307 | 2.65 µs (0.22×) | 13.6 µs (1.1×) | 12.1 µs |
| D462 | · | 21.7 µs (1.1×) | 20.5 µs |
| D616 | · | 25.8 µs (0.9×) | 28.9 µs |
| D924 | · | 39.1 µs (0.72×) | 54.7 µs |
| D1232 | · | 55.3 µs (0.66×) | 84.3 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,210.7 88.2,208.7 160.5,162.2 232.9,182.8 269.1,176.6 305.3,173.7 305.3,196.5 269.1,193.1 232.9,192.7 160.5,199.2 88.2,218.6 52.0,225.1" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,216.9 88.2,210.6 124.4,182.8 160.5,179.3 196.7,158.4 232.9,149.0 269.1,134.7 305.3,94.6 341.5,113.0 377.6,57.7 413.8,87.1 450.0,60.9 450.0,158.8 413.8,163.9 377.6,169.8 341.5,170.0 305.3,176.1 269.1,177.6 232.9,180.6 196.7,186.4 160.5,192.3 124.4,193.7 88.2,218.5 52.0,223.4" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,215.9 88.2,205.3 124.4,197.2 160.5,193.2 196.7,168.2 232.9,155.2 269.1,142.0 305.3,93.2 341.5,93.9 377.6,86.9 413.8,69.5 450.0,66.3 450.0,139.7 413.8,148.3 377.6,160.3 341.5,163.3 305.3,173.8 269.1,183.5 232.9,186.3 196.7,191.3 160.5,204.4 124.4,208.7 88.2,211.7 52.0,222.6" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,220.5 88.2,214.5 160.5,198.3 232.9,190.5 269.1,181.9 305.3,184.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,220.8 88.2,213.6 124.4,186.8 160.5,184.3 196.7,174.1 232.9,168.1 269.1,162.3 305.3,160.2 341.5,153.4 377.6,150.8 413.8,144.7 450.0,139.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.9 88.2,207.9 124.4,201.7 160.5,198.5 196.7,182.2 232.9,177.3 269.1,167.9 305.3,161.9 341.5,154.2 377.6,149.2 413.8,139.8 450.0,133.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 210 ns (0.91×) | 190 ns (0.82×) | 231 ns |
| D38 | 1.12 µs (0.85×) | 1.12 µs (0.85×) | 1.32 µs |
| D57 | · | 2.64 µs (3.3×) | 806 ns |
| D76 | · | 3.09 µs (3.1×) | 982 ns |
| D115 | · | 5.14 µs (1.9×) | 2.64 µs |
| D153 | · | 7.8 µs (2.2×) | 3.55 µs |
| D230 | 5.38 µs (0.78×) | 11.6 µs (1.7×) | 6.88 µs |
| D307 | 6.33 µs (0.66×) | 14.7 µs (1.5×) | 9.62 µs |
| D462 | · | 24 µs (1.3×) | 18.7 µs |
| D616 | · | 32 µs (1.2×) | 27.2 µs |
| D924 | · | 48.1 µs (0.99×) | 48.3 µs |
| D1232 | · | 68 µs (0.84×) | 80.7 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,217.3 88.2,187.2 269.1,158.9 305.3,156.7 305.3,172.9 269.1,172.7 88.2,190.7 52.0,222.6" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,215.9 88.2,183.4 124.4,172.2 160.5,167.4 196.7,147.3 232.9,137.2 269.1,124.8 305.3,112.7 341.5,98.9 377.6,66.0 413.8,68.2 450.0,56.2 450.0,139.9 413.8,146.0 377.6,153.1 341.5,151.4 305.3,160.9 269.1,162.8 232.9,165.6 196.7,175.6 160.5,177.9 124.4,179.9 88.2,190.7 52.0,224.9" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,212.1 88.2,184.1 124.4,191.2 160.5,186.8 196.7,161.0 232.9,151.6 269.1,133.2 305.3,119.2 341.5,87.3 377.6,91.3 413.8,71.1 450.0,39.6 450.0,121.7 413.8,130.8 377.6,142.5 341.5,148.4 305.3,160.9 269.1,172.0 232.9,179.9 196.7,182.5 160.5,198.1 124.4,202.7 88.2,189.0 52.0,219.8" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,219.0 88.2,189.6 269.1,162.1 305.3,159.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,220.7 88.2,189.6 124.4,174.6 160.5,171.8 196.7,162.9 232.9,155.6 269.1,148.6 305.3,144.4 341.5,135.8 377.6,130.8 413.8,123.7 450.0,117.6" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,217.3 88.2,186.7 124.4,195.4 160.5,191.9 196.7,174.6 232.9,169.4 269.1,157.8 305.3,151.9 341.5,140.2 377.6,133.6 413.8,123.6 450.0,114.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 260 ns (1×) | 240 ns (0.96×) | 251 ns |
| D38 | 361 ns (0.63×) | 346 ns (0.61×) | 570 ns |
| D57 | · | 2.17 µs (2.9×) | 741 ns |
| D76 | · | 2.48 µs (2.7×) | 906 ns |
| D115 | · | 4.44 µs (1.8×) | 2.43 µs |
| D153 | · | 6.73 µs (2×) | 3.4 µs |
| D230 | 3.94 µs (0.65×) | 9.03 µs (1.5×) | 6.1 µs |
| D307 | 4.98 µs (0.54×) | 11.1 µs (1.2×) | 9.24 µs |
| D462 | · | 17.6 µs (1×) | 16.8 µs |
| D616 | · | 21.8 µs (0.95×) | 23 µs |
| D924 | · | 32.5 µs (0.75×) | 43.4 µs |
| D1232 | · | 45.7 µs (0.64×) | 71 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,213.7 88.2,209.4 269.1,167.0 305.3,172.7 305.3,187.0 269.1,188.0 88.2,216.9 52.0,223.3" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,214.1 88.2,203.3 124.4,162.8 160.5,180.6 196.7,164.5 232.9,152.9 269.1,144.1 305.3,75.9 341.5,120.1 377.6,109.8 413.8,79.9 450.0,58.4 450.0,158.4 413.8,162.3 377.6,168.3 341.5,169.3 305.3,175.9 269.1,176.5 232.9,180.8 196.7,185.5 160.5,190.7 124.4,191.7 88.2,216.4 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,214.1 88.2,201.4 124.4,176.7 160.5,194.6 196.7,173.7 232.9,147.3 269.1,150.0 305.3,135.4 341.5,89.6 377.6,81.9 413.8,66.8 450.0,62.3 450.0,145.4 413.8,153.7 377.6,156.7 341.5,162.9 305.3,174.0 269.1,178.5 232.9,185.4 196.7,189.8 160.5,203.5 124.4,206.3 88.2,209.7 52.0,220.4" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,218.0 88.2,213.2 269.1,178.3 305.3,174.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,219.2 88.2,213.9 124.4,187.0 160.5,185.0 196.7,176.5 232.9,170.5 269.1,166.2 305.3,163.1 341.5,156.4 377.6,153.3 413.8,147.4 450.0,142.5" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.5 88.2,206.6 124.4,202.7 160.5,199.8 196.7,185.4 232.9,180.4 269.1,171.9 305.3,165.8 341.5,157.1 377.6,152.5 413.8,143.2 450.0,136.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 215 ns (0.88×) | 196 ns (0.8×) | 245 ns |
| D38 | 340 ns (0.67×) | 350 ns (0.68×) | 511 ns |
| D57 | · | 2.14 µs (3.1×) | 681 ns |
| D76 | · | 2.52 µs (2.8×) | 891 ns |
| D115 | · | 4.33 µs (1.8×) | 2.47 µs |
| D153 | · | 6.64 µs (1.9×) | 3.42 µs |
| D230 | 3.37 µs (0.54×) | 9.01 µs (1.4×) | 6.27 µs |
| D307 | 4.86 µs (0.48×) | 11.2 µs (1.1×) | 10.2 µs |
| D462 | · | 16.6 µs (0.88×) | 18.9 µs |
| D616 | · | 18.9 µs (0.7×) | 27.1 µs |
| D924 | · | 33.4 µs (0.64×) | 52 µs |
| D1232 | · | 40 µs (0.46×) | 86.6 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,218.6 88.2,211.4 269.1,177.6 305.3,174.3 305.3,192.1 269.1,195.1 88.2,217.4 52.0,221.9" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,220.5 88.2,211.0 124.4,186.2 160.5,183.6 196.7,169.1 232.9,151.6 269.1,148.3 305.3,140.3 341.5,125.1 377.6,66.3 413.8,60.7 450.0,58.9 450.0,158.6 413.8,163.7 377.6,170.0 341.5,169.5 305.3,174.7 269.1,176.7 232.9,180.1 196.7,190.7 160.5,193.4 124.4,194.0 88.2,216.4 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,217.5 88.2,206.5 124.4,200.9 160.5,196.1 196.7,175.0 232.9,166.3 269.1,146.0 305.3,134.3 341.5,107.0 377.6,109.8 413.8,86.0 450.0,68.7 450.0,143.4 413.8,153.1 377.6,161.3 341.5,164.6 305.3,172.2 269.1,179.2 232.9,187.7 196.7,192.5 160.5,204.1 124.4,209.0 88.2,212.8 52.0,221.2" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,220.8 88.2,214.1 269.1,180.6 305.3,175.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,222.2 88.2,213.7 124.4,187.2 160.5,184.8 196.7,176.9 232.9,170.7 269.1,166.2 305.3,163.1 341.5,157.2 377.6,155.4 413.8,147.0 450.0,144.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.9 88.2,208.1 124.4,204.0 160.5,200.0 196.7,185.1 232.9,180.4 269.1,171.5 305.3,164.4 341.5,155.3 377.6,150.1 413.8,140.6 450.0,133.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 236 ns (0.93×) | 210 ns (0.82×) | 255 ns |
| D38 | 331 ns (0.65×) | 321 ns (0.63×) | 511 ns |
| D57 | · | 2.2 µs (2.8×) | 781 ns |
| D76 | 1.1 µs (1.2×) | 2.58 µs (2.8×) | 922 ns |
| D115 | · | 5.15 µs (1.7×) | 2.99 µs |
| D153 | 2.25 µs (0.53×) | 8.14 µs (1.9×) | 4.21 µs |
| D230 | 3.19 µs (0.4×) | 11.9 µs (1.5×) | 7.92 µs |
| D307 | 4.03 µs (0.35×) | 14.2 µs (1.2×) | 11.6 µs |
| D462 | · | 21.5 µs (1×) | 20.5 µs |
| D616 | · | 25.6 µs (0.85×) | 30 µs |
| D924 | · | 40.3 µs (0.76×) | 53.2 µs |
| D1232 | · | 51.1 µs (0.59×) | 87.1 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,163.2 88.2,211.4 160.5,196.3 232.9,184.0 269.1,177.4 305.3,173.7 305.3,194.3 269.1,194.9 232.9,193.7 160.5,206.0 88.2,219.1 52.0,223.4" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,216.9 88.2,206.0 124.4,183.2 160.5,179.3 196.7,160.0 232.9,141.1 269.1,136.8 305.3,126.6 341.5,113.2 377.6,103.1 413.8,87.0 450.0,60.8 450.0,159.0 413.8,164.0 377.6,170.6 341.5,171.4 305.3,177.4 269.1,178.0 232.9,180.4 196.7,189.4 160.5,196.1 124.4,196.9 88.2,219.2 52.0,223.4" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,212.9 88.2,205.8 124.4,195.2 160.5,192.1 196.7,163.6 232.9,150.0 269.1,141.8 305.3,127.3 341.5,110.5 377.6,98.5 413.8,85.3 450.0,70.9 450.0,140.2 413.8,148.2 377.6,156.8 341.5,163.4 305.3,173.4 269.1,182.9 232.9,186.4 196.7,192.3 160.5,206.0 124.4,211.0 88.2,214.1 52.0,222.6" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,219.4 88.2,214.5 160.5,196.9 232.9,186.5 269.1,181.4 305.3,178.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.2 88.2,214.9 124.4,186.8 160.5,184.5 196.7,174.4 232.9,167.7 269.1,162.1 305.3,159.5 341.5,153.5 377.6,150.9 413.8,144.3 450.0,140.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.3 88.2,208.1 124.4,201.9 160.5,199.5 196.7,182.3 232.9,177.3 269.1,168.1 305.3,162.5 341.5,154.2 377.6,148.6 413.8,140.2 450.0,133.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body:arithmetic -->
