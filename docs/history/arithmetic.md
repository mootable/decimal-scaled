# History — Arithmetic

How the arithmetic operations have moved release over release. See the
[History overview](../history.md) for the time units, the width reference map, and how
these timings are measured.

<!-- BEGIN GENERATED:history:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 230 ns (1×) | 205 ns (0.89×) | 231 ns |
| D38 | 341 ns (0.68×) | 340 ns (0.68×) | 500 ns |
| D57 | · | 2.2 µs (2.8×) | 781 ns |
| D76 | 992 ns (0.97×) | 2.6 µs (2.5×) | 1.02 µs |
| D115 | · | 4.87 µs (2×) | 2.48 µs |
| D153 | 1.61 µs (0.38×) | 8.1 µs (1.9×) | 4.22 µs |
| D230 | 3.03 µs (0.37×) | 11.6 µs (1.4×) | 8.14 µs |
| D307 | 3.8 µs (0.34×) | 12.7 µs (1.1×) | 11.3 µs |
| D462 | · | 22 µs (1×) | 21.8 µs |
| D616 | · | 22.7 µs (0.92×) | 24.7 µs |
| D924 | · | 41.9 µs (0.77×) | 54.5 µs |
| D1232 | · | 52.5 µs (0.61×) | 86.1 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,210.0 88.2,208.1 160.5,194.0 232.9,182.0 269.1,153.6 305.3,174.3 305.3,189.2 269.1,193.4 232.9,192.3 160.5,199.1 88.2,218.6 52.0,224.2" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,218.0 88.2,211.0 124.4,183.0 160.5,179.6 196.7,148.9 232.9,148.3 269.1,135.8 305.3,129.1 341.5,112.3 377.6,68.8 413.8,87.6 450.0,60.7 450.0,160.4 413.8,162.2 377.6,171.6 341.5,170.7 305.3,176.0 269.1,177.3 232.9,180.7 196.7,190.4 160.5,192.2 124.4,193.7 88.2,216.9 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,215.4 88.2,206.3 124.4,198.0 160.5,191.9 196.7,169.5 232.9,157.7 269.1,141.8 305.3,114.9 341.5,91.3 377.6,107.2 413.8,72.8 450.0,63.5 450.0,146.9 413.8,148.0 377.6,160.7 341.5,163.0 305.3,173.0 269.1,183.2 232.9,189.1 196.7,194.9 160.5,203.7 124.4,209.3 88.2,212.8 52.0,221.2" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,219.8 88.2,214.1 160.5,198.5 232.9,191.4 269.1,182.1 305.3,178.8" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.5 88.2,214.1 124.4,186.8 160.5,184.3 196.7,175.2 232.9,167.7 269.1,162.5 305.3,161.2 341.5,153.1 377.6,152.7 413.8,143.7 450.0,140.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.8 88.2,208.5 124.4,201.9 160.5,198.0 196.7,185.0 232.9,177.3 269.1,167.7 305.3,162.8 341.5,153.3 377.6,151.5 413.8,139.9 450.0,133.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 201 ns (0.84×) | 190 ns (0.79×) | 240 ns |
| D38 | 1.14 µs (0.88×) | 1.12 µs (0.86×) | 1.3 µs |
| D57 | · | 2.64 µs (3.3×) | 796 ns |
| D76 | · | 3.1 µs (3.1×) | 1.01 µs |
| D115 | · | 4.73 µs (2.1×) | 2.24 µs |
| D153 | · | 7.99 µs (2.2×) | 3.6 µs |
| D230 | 5.04 µs (0.76×) | 12 µs (1.8×) | 6.62 µs |
| D307 | 6.42 µs (0.67×) | 15.1 µs (1.6×) | 9.56 µs |
| D462 | · | 25.8 µs (1.3×) | 19.2 µs |
| D616 | · | 24.6 µs (1.1×) | 22.6 µs |
| D924 | · | 55.2 µs (0.98×) | 56.1 µs |
| D1232 | · | 70.6 µs (0.83×) | 85.1 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,218.5 88.2,194.9 269.1,173.1 305.3,149.8 305.3,179.2 269.1,182.0 88.2,197.5 52.0,225.1" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,221.2 88.2,191.9 124.4,182.0 160.5,178.2 196.7,162.6 232.9,152.8 269.1,142.6 305.3,134.9 341.5,94.9 377.6,112.9 413.8,96.1 450.0,59.3 450.0,156.2 413.8,157.0 377.6,166.7 341.5,166.6 305.3,172.5 269.1,174.8 232.9,175.8 196.7,182.0 160.5,187.0 124.4,188.7 88.2,197.6 52.0,225.1" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,215.9 88.2,191.8 124.4,199.2 160.5,193.7 196.7,175.0 232.9,164.8 269.1,147.2 305.3,133.7 341.5,124.2 377.6,117.6 413.8,70.4 450.0,65.4 450.0,139.5 413.8,147.5 377.6,166.5 341.5,162.4 305.3,172.9 269.1,178.3 232.9,188.8 196.7,195.4 160.5,204.0 124.4,208.1 88.2,196.0 52.0,222.5" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.8 88.2,196.4 269.1,174.7 305.3,171.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,222.6 88.2,196.7 124.4,184.1 160.5,181.8 196.7,175.6 232.9,167.9 269.1,162.0 305.3,158.7 341.5,150.8 377.6,151.5 413.8,139.7 450.0,136.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.2 88.2,194.5 124.4,201.7 160.5,198.2 196.7,186.5 232.9,179.6 269.1,170.7 305.3,165.3 341.5,155.1 377.6,152.7 413.8,139.5 450.0,133.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 241 ns (0.89×) | 230 ns (0.85×) | 270 ns |
| D38 | 361 ns (0.64×) | 360 ns (0.64×) | 561 ns |
| D57 | · | 2.15 µs (3×) | 706 ns |
| D76 | · | 2.48 µs (2.7×) | 907 ns |
| D115 | · | 4.12 µs (1.9×) | 2.15 µs |
| D153 | · | 6.89 µs (2×) | 3.44 µs |
| D230 | 4.04 µs (0.66×) | 8.98 µs (1.5×) | 6.14 µs |
| D307 | 4.93 µs (0.55×) | 11.1 µs (1.2×) | 8.93 µs |
| D462 | · | 18.4 µs (1.1×) | 17.5 µs |
| D616 | · | 18.4 µs (0.93×) | 19.9 µs |
| D924 | · | 34.8 µs (0.78×) | 44.5 µs |
| D1232 | · | 44.4 µs (0.62×) | 71.3 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,215.5 88.2,208.4 269.1,175.3 305.3,172.9 305.3,184.7 269.1,187.9 88.2,215.5 52.0,225.1" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,213.7 88.2,203.5 124.4,184.3 160.5,180.5 196.7,145.2 232.9,153.1 269.1,143.7 305.3,135.8 341.5,116.9 377.6,64.9 413.8,96.9 450.0,60.9 450.0,159.2 413.8,159.8 377.6,170.5 341.5,169.6 305.3,173.8 269.1,177.4 232.9,180.0 196.7,186.7 160.5,190.8 124.4,192.0 88.2,217.4 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,213.3 88.2,201.0 124.4,161.2 160.5,193.8 196.7,175.7 232.9,164.1 269.1,150.6 305.3,136.6 341.5,86.7 377.6,115.6 413.8,70.6 450.0,95.6 450.0,139.3 413.8,146.6 377.6,159.0 341.5,167.6 305.3,173.5 269.1,177.6 232.9,185.0 196.7,191.9 160.5,203.1 124.4,206.8 88.2,210.6 52.0,221.1" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,219.1 88.2,213.2 269.1,177.9 305.3,175.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,219.8 88.2,213.3 124.4,187.1 160.5,185.1 196.7,177.6 232.9,170.1 269.1,166.2 305.3,163.2 341.5,155.8 377.6,155.7 413.8,146.4 450.0,142.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,217.5 88.2,206.8 124.4,203.4 160.5,199.8 196.7,187.1 232.9,180.3 269.1,171.8 305.3,166.3 341.5,156.5 377.6,154.6 413.8,142.8 450.0,135.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 210 ns (0.88×) | 210 ns (0.88×) | 240 ns |
| D38 | 340 ns (0.67×) | 341 ns (0.67×) | 510 ns |
| D57 | · | 2.19 µs (3.3×) | 661 ns |
| D76 | · | 2.52 µs (2.8×) | 912 ns |
| D115 | · | 4.06 µs (2×) | 2.05 µs |
| D153 | · | 6.62 µs (1.9×) | 3.46 µs |
| D230 | 3.44 µs (0.54×) | 9.03 µs (1.4×) | 6.33 µs |
| D307 | 4.54 µs (0.49×) | 10.9 µs (1.2×) | 9.26 µs |
| D462 | · | 17.3 µs (0.92×) | 18.8 µs |
| D616 | · | 16.6 µs (0.73×) | 22.8 µs |
| D924 | · | 34.4 µs (0.68×) | 50.9 µs |
| D1232 | · | 46.7 µs (0.55×) | 84.8 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,219.1 88.2,210.3 269.1,177.3 305.3,174.5 305.3,187.5 269.1,190.7 88.2,216.4 52.0,223.4" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,218.6 88.2,210.3 124.4,186.1 160.5,183.2 196.7,169.4 232.9,159.0 269.1,147.6 305.3,140.6 341.5,75.1 377.6,117.3 413.8,64.4 450.0,58.8 450.0,160.5 413.8,162.1 377.6,170.6 341.5,170.3 305.3,175.0 269.1,177.1 232.9,180.2 196.7,188.2 160.5,193.6 124.4,195.4 88.2,216.4 52.0,221.9" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,217.4 88.2,205.5 124.4,201.8 160.5,195.2 196.7,176.9 232.9,166.2 269.1,150.7 305.3,100.4 341.5,109.7 377.6,117.2 413.8,99.2 450.0,65.7 450.0,144.7 413.8,152.2 377.6,163.2 341.5,163.5 305.3,174.0 269.1,179.1 232.9,189.6 196.7,198.3 160.5,204.2 124.4,210.0 88.2,212.1 52.0,220.4" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.2 88.2,214.1 269.1,180.2 305.3,176.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.2 88.2,214.1 124.4,186.8 160.5,184.8 196.7,177.8 232.9,170.7 269.1,166.2 305.3,163.4 341.5,156.6 377.6,157.3 413.8,146.6 450.0,142.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.2 88.2,208.2 124.4,204.4 160.5,199.7 196.7,187.9 232.9,180.2 269.1,171.3 305.3,165.8 341.5,155.5 377.6,152.6 413.8,140.9 450.0,133.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 190 ns (0.83×) | 205 ns (0.89×) | 230 ns |
| D38 | 330 ns (0.7×) | 331 ns (0.7×) | 471 ns |
| D57 | · | 2.18 µs (2.9×) | 751 ns |
| D76 | 1.12 µs (1.2×) | 2.62 µs (2.7×) | 962 ns |
| D115 | · | 4.9 µs (2×) | 2.44 µs |
| D153 | 2.26 µs (0.54×) | 8.26 µs (2×) | 4.23 µs |
| D230 | 3.4 µs (0.42×) | 11.6 µs (1.4×) | 8.13 µs |
| D307 | 4.05 µs (0.36×) | 13.5 µs (1.2×) | 11.2 µs |
| D462 | · | 22.3 µs (1×) | 21.5 µs |
| D616 | · | 22.2 µs (0.94×) | 23.7 µs |
| D924 | · | 42 µs (0.8×) | 52.2 µs |
| D1232 | · | 52.7 µs (0.59×) | 89.3 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,218.0 88.2,211.7 160.5,196.3 232.9,184.5 269.1,177.5 305.3,174.5 305.3,193.2 269.1,193.3 232.9,195.3 160.5,206.5 88.2,219.2 52.0,224.2" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,217.5 88.2,206.8 124.4,182.9 160.5,159.8 196.7,148.6 232.9,147.0 269.1,135.3 305.3,105.4 341.5,111.4 377.6,105.0 413.8,87.6 450.0,73.5 450.0,160.3 413.8,162.2 377.6,172.4 341.5,170.9 305.3,177.6 269.1,178.4 232.9,181.5 196.7,190.9 160.5,195.8 124.4,198.5 88.2,219.8 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,216.4 88.2,206.5 124.4,196.1 160.5,191.6 196.7,170.2 232.9,158.0 269.1,138.5 305.3,127.7 341.5,86.6 377.6,108.5 413.8,77.5 450.0,58.0 450.0,140.4 413.8,147.8 377.6,160.6 341.5,163.3 305.3,173.2 269.1,182.4 232.9,188.8 196.7,195.6 160.5,206.5 124.4,211.7 88.2,214.5 52.0,221.2" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,222.6 88.2,214.5 160.5,196.7 232.9,186.4 269.1,180.4 305.3,177.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.5 88.2,214.5 124.4,186.9 160.5,184.3 196.7,175.1 232.9,167.5 269.1,162.5 305.3,160.3 341.5,153.0 377.6,153.0 413.8,143.7 450.0,140.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.8 88.2,209.3 124.4,202.5 160.5,198.9 196.7,185.3 232.9,177.3 269.1,167.7 305.3,163.0 341.5,153.5 377.6,152.0 413.8,140.5 450.0,132.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body:arithmetic -->
