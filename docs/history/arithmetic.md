# History — Arithmetic

How the arithmetic operations have moved release over release. See the
[History overview](../history.md) for the time units, the width reference map, and how
these timings are measured.

<!-- BEGIN GENERATED:history:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 210 ns (0.84×) | 200 ns (0.8×) | 250 ns |
| D38 | 350 ns (0.68×) | 350 ns (0.68×) | 511 ns |
| D57 | · | 2.23 µs (3×) | 752 ns |
| D76 | 982 ns (0.96×) | 2.6 µs (2.5×) | 1.02 µs |
| D115 | · | 5.31 µs (1.8×) | 3 µs |
| D153 | 2.05 µs (0.49×) | 7.85 µs (1.9×) | 4.19 µs |
| D230 | 2.05 µs (0.25×) | 11.8 µs (1.5×) | 8.12 µs |
| D307 | 4.09 µs (0.36×) | 13.4 µs (1.2×) | 11.2 µs |
| D462 | · | 21.5 µs (0.99×) | 21.8 µs |
| D616 | · | 28.2 µs (0.9×) | 31.4 µs |
| D924 | · | 38.4 µs (0.69×) | 56.1 µs |
| D1232 | · | 59.2 µs (0.65×) | 91.7 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,213.7 88.2,208.4 160.5,194.1 232.9,181.5 269.1,178.3 305.3,150.2 305.3,190.0 269.1,199.1 232.9,190.1 160.5,199.1 88.2,215.9 52.0,225.0" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,215.0 88.2,210.3 124.4,182.9 160.5,179.7 196.7,159.7 232.9,150.0 269.1,104.4 305.3,89.3 341.5,111.3 377.6,62.2 413.8,87.9 450.0,60.6 450.0,159.3 413.8,163.1 377.6,168.1 341.5,169.9 305.3,175.5 269.1,177.9 232.9,179.8 196.7,186.2 160.5,192.0 124.4,194.3 88.2,218.5 52.0,223.4" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,212.4 88.2,205.5 124.4,197.9 160.5,193.3 196.7,168.6 232.9,158.2 269.1,140.9 305.3,92.7 341.5,94.8 377.6,103.1 413.8,67.4 450.0,63.0 450.0,139.9 413.8,147.2 377.6,156.7 341.5,163.3 305.3,174.3 269.1,183.3 232.9,188.5 196.7,197.3 160.5,204.2 124.4,209.3 88.2,212.8 52.0,219.8" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.2 88.2,213.7 160.5,198.6 232.9,187.8 269.1,187.9 305.3,177.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.9 88.2,213.7 124.4,186.6 160.5,184.4 196.7,173.9 232.9,168.2 269.1,162.3 305.3,160.4 341.5,153.5 377.6,149.5 413.8,145.0 450.0,138.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.6 88.2,208.1 124.4,202.5 160.5,198.0 196.7,182.3 232.9,177.4 269.1,167.7 305.3,163.0 341.5,153.3 377.6,147.9 413.8,139.5 450.0,132.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 200 ns (0.87×) | 190 ns (0.83×) | 230 ns |
| D38 | 1.14 µs (0.88×) | 1.12 µs (0.87×) | 1.29 µs |
| D57 | · | 2.63 µs (3.4×) | 786 ns |
| D76 | · | 3.11 µs (3.1×) | 1.01 µs |
| D115 | · | 4.99 µs (2×) | 2.52 µs |
| D153 | · | 7.88 µs (2.2×) | 3.65 µs |
| D230 | 5.27 µs (0.78×) | 12.1 µs (1.8×) | 6.76 µs |
| D307 | 6.44 µs (0.67×) | 14 µs (1.5×) | 9.62 µs |
| D462 | · | 25.8 µs (1.3×) | 19.1 µs |
| D616 | · | 34 µs (1.2×) | 28.5 µs |
| D924 | · | 47.1 µs (0.83×) | 56.6 µs |
| D1232 | · | 69 µs (0.77×) | 89.5 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,219.2 88.2,194.8 269.1,172.9 305.3,168.8 305.3,180.5 269.1,182.9 88.2,197.7 52.0,225.1" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,220.5 88.2,191.4 124.4,181.9 160.5,178.0 196.7,161.0 232.9,154.2 269.1,141.9 305.3,134.0 341.5,117.8 377.6,109.8 413.8,96.3 450.0,58.3 450.0,156.2 413.8,158.2 377.6,164.1 341.5,165.0 305.3,172.4 269.1,173.8 232.9,175.7 196.7,184.5 160.5,187.0 124.4,188.5 88.2,197.7 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,216.9 88.2,192.0 124.4,198.9 160.5,194.1 196.7,172.5 232.9,165.1 269.1,149.3 305.3,99.2 341.5,106.2 377.6,112.8 413.8,80.2 450.0,67.6 450.0,139.3 413.8,146.7 377.6,156.5 341.5,162.3 305.3,172.8 269.1,178.1 232.9,188.8 196.7,197.7 160.5,203.7 124.4,207.6 88.2,196.2 52.0,221.9" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.9 88.2,196.4 269.1,174.0 305.3,171.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,222.6 88.2,196.7 124.4,184.2 160.5,181.8 196.7,174.8 232.9,168.2 269.1,161.9 305.3,159.8 341.5,150.8 377.6,146.8 413.8,142.0 450.0,136.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.8 88.2,194.6 124.4,201.9 160.5,198.2 196.7,184.8 232.9,179.4 269.1,170.4 305.3,165.2 341.5,155.2 377.6,149.3 413.8,139.3 450.0,132.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 251 ns (0.93×) | 221 ns (0.82×) | 270 ns |
| D38 | 360 ns (0.68×) | 351 ns (0.66×) | 531 ns |
| D57 | · | 2.13 µs (3×) | 721 ns |
| D76 | · | 2.51 µs (2.8×) | 902 ns |
| D115 | · | 4.39 µs (1.8×) | 2.45 µs |
| D153 | · | 6.64 µs (1.9×) | 3.45 µs |
| D230 | 4.11 µs (0.63×) | 9.18 µs (1.4×) | 6.5 µs |
| D307 | 5.12 µs (0.55×) | 10.7 µs (1.2×) | 9.27 µs |
| D462 | · | 18.7 µs (1×) | 17.9 µs |
| D616 | · | 23.6 µs (0.95×) | 24.9 µs |
| D924 | · | 31.6 µs (0.7×) | 45.2 µs |
| D1232 | · | 44.6 µs (0.57×) | 78.2 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,214.9 88.2,208.4 269.1,176.2 305.3,152.6 305.3,183.3 269.1,186.9 88.2,216.9 52.0,222.6" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,215.4 88.2,203.7 124.4,183.7 160.5,180.7 196.7,152.4 232.9,154.4 269.1,143.3 305.3,135.9 341.5,65.1 377.6,60.8 413.8,97.2 450.0,68.1 450.0,159.4 413.8,163.5 377.6,167.6 341.5,168.9 305.3,175.7 269.1,176.7 232.9,179.1 196.7,185.0 160.5,190.8 124.4,191.8 88.2,216.9 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,214.1 88.2,161.8 124.4,187.2 160.5,194.4 196.7,173.2 232.9,164.4 269.1,149.4 305.3,129.5 341.5,97.8 377.6,80.8 413.8,63.8 450.0,63.5 450.0,139.2 413.8,149.5 377.6,156.7 341.5,162.6 305.3,172.0 269.1,177.3 232.9,185.2 196.7,189.8 160.5,202.7 124.4,207.0 88.2,210.3 52.0,219.8" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,218.5 88.2,213.3 269.1,177.7 305.3,174.4" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,220.4 88.2,213.6 124.4,187.3 160.5,184.9 196.7,176.7 232.9,170.6 269.1,165.9 305.3,163.7 341.5,155.5 377.6,152.1 413.8,147.9 450.0,142.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,217.5 88.2,207.6 124.4,203.1 160.5,199.8 196.7,185.2 232.9,180.2 269.1,171.0 305.3,165.8 341.5,156.1 377.6,151.4 413.8,142.6 450.0,134.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 210 ns (0.93×) | 201 ns (0.89×) | 225 ns |
| D38 | 350 ns (0.69×) | 346 ns (0.68×) | 506 ns |
| D57 | · | 2.17 µs (3.2×) | 671 ns |
| D76 | · | 2.52 µs (2.8×) | 902 ns |
| D115 | · | 4.32 µs (1.8×) | 2.45 µs |
| D153 | · | 6.66 µs (1.9×) | 3.53 µs |
| D230 | 3.8 µs (0.57×) | 8.98 µs (1.4×) | 6.65 µs |
| D307 | 4.61 µs (0.49×) | 11.2 µs (1.2×) | 9.44 µs |
| D462 | · | 18.5 µs (0.88×) | 21 µs |
| D616 | · | 21.1 µs (0.71×) | 29.8 µs |
| D924 | · | 29.5 µs (0.54×) | 54.6 µs |
| D1232 | · | 46.1 µs (0.53×) | 87.1 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,218.6 88.2,210.0 269.1,178.2 305.3,174.3 305.3,187.7 269.1,190.4 88.2,217.4 52.0,221.9" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,219.8 88.2,211.4 124.4,186.2 160.5,183.5 196.7,168.0 232.9,159.9 269.1,146.9 305.3,96.6 341.5,101.1 377.6,113.6 413.8,99.6 450.0,59.4 450.0,159.8 413.8,162.5 377.6,168.9 341.5,169.5 305.3,175.0 269.1,177.9 232.9,180.4 196.7,191.9 160.5,193.6 124.4,195.8 88.2,216.4 52.0,223.4" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,218.0 88.2,204.4 124.4,201.2 160.5,196.1 196.7,175.1 232.9,166.2 269.1,150.0 305.3,133.9 341.5,122.6 377.6,112.8 413.8,76.1 450.0,64.2 450.0,138.7 413.8,145.7 377.6,160.4 341.5,162.7 305.3,173.5 269.1,182.0 232.9,187.7 196.7,195.5 160.5,204.4 124.4,208.7 88.2,212.8 52.0,220.5" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.2 88.2,213.7 269.1,178.8 305.3,176.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.8 88.2,213.9 124.4,187.0 160.5,184.8 196.7,176.9 232.9,170.6 269.1,166.2 305.3,163.1 341.5,155.7 377.6,153.8 413.8,148.8 450.0,142.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,220.1 88.2,208.3 124.4,204.2 160.5,199.8 196.7,185.2 232.9,179.9 269.1,170.6 305.3,165.5 341.5,153.8 377.6,148.7 413.8,139.8 450.0,133.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 196 ns (0.8×) | 200 ns (0.82×) | 245 ns |
| D38 | 340 ns (0.72×) | 320 ns (0.68×) | 471 ns |
| D57 | · | 2.15 µs (2.9×) | 742 ns |
| D76 | 1.13 µs (1.2×) | 2.6 µs (2.7×) | 952 ns |
| D115 | · | 5.21 µs (1.8×) | 2.94 µs |
| D153 | 2.41 µs (0.56×) | 7.96 µs (1.9×) | 4.3 µs |
| D230 | 3.59 µs (0.44×) | 11.6 µs (1.4×) | 8.11 µs |
| D307 | 4.06 µs (0.36×) | 13.3 µs (1.2×) | 11.2 µs |
| D462 | · | 22.3 µs (1×) | 21.4 µs |
| D616 | · | 28.1 µs (0.91×) | 30.9 µs |
| D924 | · | 37.7 µs (0.68×) | 55.8 µs |
| D1232 | · | 54.2 µs (0.62×) | 87.8 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,215.9 88.2,211.3 160.5,196.1 232.9,183.8 269.1,177.9 305.3,174.5 305.3,191.2 269.1,191.9 232.9,192.5 160.5,207.0 88.2,219.2 52.0,225.1" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,217.5 88.2,207.3 124.4,182.5 160.5,179.4 196.7,158.6 232.9,150.2 269.1,135.3 305.3,85.7 341.5,66.9 377.6,101.6 413.8,64.6 450.0,48.2 450.0,159.6 413.8,163.9 377.6,168.7 341.5,170.3 305.3,177.5 269.1,178.6 232.9,180.7 196.7,186.7 160.5,196.3 124.4,198.8 88.2,218.5 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,214.5 88.2,206.0 124.4,195.6 160.5,192.1 196.7,168.8 232.9,158.1 269.1,140.9 305.3,101.6 341.5,90.6 377.6,76.7 413.8,65.2 450.0,62.1 450.0,139.3 413.8,151.4 377.6,156.9 341.5,162.5 305.3,174.0 269.1,178.8 232.9,189.3 196.7,199.1 160.5,207.0 124.4,211.4 88.2,214.5 52.0,221.1" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,222.2 88.2,214.1 160.5,196.6 232.9,185.5 269.1,179.7 305.3,177.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.9 88.2,215.0 124.4,187.1 160.5,184.3 196.7,174.2 232.9,168.0 269.1,162.5 305.3,160.5 341.5,152.9 377.6,149.6 413.8,145.3 450.0,139.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.9 88.2,209.3 124.4,202.7 160.5,199.1 196.7,182.6 232.9,177.0 269.1,167.7 305.3,163.0 341.5,153.6 377.6,148.1 413.8,139.5 450.0,132.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body:arithmetic -->
