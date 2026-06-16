# History — Arithmetic

How the arithmetic operations have moved release over release. See the
[History overview](../history.md) for the time units, the width reference map, and how
these timings are measured.

<!-- BEGIN GENERATED:history:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 219 ns (0.86×) | 212 ns (0.83×) | 254 ns |
| D38 | 344 ns (0.68×) | 343 ns (0.68×) | 507 ns |
| D57 | · | 1.88 µs (2.4×) | 777 ns |
| D76 | 892 ns (0.86×) | 2.68 µs (2.6×) | 1.04 µs |
| D115 | · | 5.18 µs (2×) | 2.61 µs |
| D153 | 1.93 µs (0.45×) | 7.94 µs (1.9×) | 4.27 µs |
| D230 | 3.21 µs (0.42×) | 11.1 µs (1.5×) | 7.67 µs |
| D307 | 4 µs (0.33×) | 14.4 µs (1.2×) | 12.3 µs |
| D462 | · | 20.8 µs (1×) | 20.9 µs |
| D616 | · | 28.5 µs (0.92×) | 30.9 µs |
| D924 | · | 41.6 µs (0.68×) | 61 µs |
| D1232 | · | 54 µs (0.63×) | 85.7 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,212.7 88.2,205.3 160.5,194.0 232.9,182.2 269.1,178.4 305.3,174.5 305.3,189.4 269.1,189.9 232.9,189.8 160.5,200.9 88.2,219.5 52.0,223.2" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,216.8 88.2,209.9 124.4,185.2 160.5,178.2 196.7,159.8 232.9,148.7 269.1,137.8 305.3,126.5 341.5,83.5 377.6,76.9 413.8,60.3 450.0,73.7 450.0,160.1 413.8,163.6 377.6,168.1 341.5,169.6 305.3,175.1 269.1,177.3 232.9,180.3 196.7,185.9 160.5,192.7 124.4,198.1 88.2,218.0 52.0,222.9" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,211.3 88.2,206.2 124.4,196.6 160.5,192.7 196.7,170.1 232.9,139.4 269.1,142.9 305.3,121.1 341.5,86.9 377.6,75.1 413.8,70.5 450.0,72.0 450.0,144.4 413.8,147.3 377.6,156.5 341.5,163.3 305.3,172.6 269.1,179.1 232.9,185.5 196.7,195.2 160.5,205.1 124.4,210.4 88.2,211.6 52.0,222.2" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,220.5 88.2,213.9 160.5,200.0 232.9,188.7 269.1,181.3 305.3,178.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.0 88.2,214.0 124.4,189.1 160.5,183.9 196.7,174.3 232.9,168.0 269.1,163.1 305.3,159.3 341.5,153.9 377.6,149.4 413.8,143.8 450.0,140.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.4 88.2,208.3 124.4,202.0 160.5,197.7 196.7,184.3 232.9,177.1 269.1,168.5 305.3,161.7 341.5,153.9 377.6,148.2 413.8,138.2 450.0,133.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 216 ns (0.98×) | 187 ns (0.85×) | 221 ns |
| D38 | 804 ns (0.8×) | 835 ns (0.83×) | 1.01 µs |
| D57 | · | 2.16 µs (2.7×) | 807 ns |
| D76 | · | 3.21 µs (3.1×) | 1.03 µs |
| D115 | · | 5.29 µs (2.4×) | 2.24 µs |
| D153 | · | 7.82 µs (2.2×) | 3.48 µs |
| D230 | 5.36 µs (0.83×) | 11.4 µs (1.8×) | 6.49 µs |
| D307 | 6.54 µs (0.61×) | 16 µs (1.5×) | 10.7 µs |
| D462 | · | 25 µs (1.4×) | 17.7 µs |
| D616 | · | 31.1 µs (1×) | 29.6 µs |
| D924 | · | 50 µs (0.89×) | 56.1 µs |
| D1232 | · | 68.1 µs (0.81×) | 84 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,218.9 88.2,199.6 269.1,171.8 305.3,169.6 305.3,180.8 269.1,179.4 88.2,204.1 52.0,222.9" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,217.4 88.2,195.0 124.4,184.0 160.5,177.7 196.7,161.8 232.9,154.1 269.1,90.0 305.3,133.7 341.5,118.4 377.6,109.5 413.8,61.4 450.0,85.9 450.0,156.0 413.8,160.0 377.6,163.4 341.5,165.3 305.3,171.4 269.1,174.1 232.9,174.9 196.7,181.3 160.5,186.7 124.4,192.0 88.2,202.7 52.0,224.3" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,214.8 88.2,195.4 124.4,197.7 160.5,194.4 196.7,176.5 232.9,165.3 269.1,150.6 305.3,136.0 341.5,125.5 377.6,96.4 413.8,98.6 450.0,79.8 450.0,139.7 413.8,146.8 377.6,159.6 341.5,162.3 305.3,173.2 269.1,182.9 232.9,188.6 196.7,196.0 160.5,204.2 124.4,208.5 88.2,201.5 52.0,222.7" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,220.7 88.2,201.5 269.1,173.8 305.3,170.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,222.8 88.2,201.0 124.4,187.1 160.5,181.3 196.7,174.0 232.9,168.3 269.1,162.7 305.3,157.8 341.5,151.3 377.6,148.1 413.8,141.1 450.0,136.6" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,220.4 88.2,198.2 124.4,201.5 160.5,197.9 196.7,186.5 232.9,180.1 269.1,171.0 305.3,163.7 341.5,156.3 377.6,148.8 413.8,139.5 450.0,133.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 237 ns (0.93×) | 239 ns (0.93×) | 256 ns |
| D38 | 378 ns (0.69×) | 370 ns (0.68×) | 547 ns |
| D57 | · | 1.78 µs (2.4×) | 740 ns |
| D76 | · | 2.58 µs (2.7×) | 946 ns |
| D115 | · | 4.36 µs (2.1×) | 2.03 µs |
| D153 | · | 6.56 µs (1.9×) | 3.47 µs |
| D230 | 4.21 µs (0.68×) | 8.59 µs (1.4×) | 6.17 µs |
| D307 | 5.07 µs (0.52×) | 11.7 µs (1.2×) | 9.79 µs |
| D462 | · | 18.2 µs (1.1×) | 17.1 µs |
| D616 | · | 23.8 µs (0.9×) | 26.5 µs |
| D924 | · | 33.6 µs (0.68×) | 49.1 µs |
| D1232 | · | 43.9 µs (0.6×) | 73.4 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,210.3 88.2,206.7 269.1,175.6 305.3,172.8 305.3,186.4 269.1,185.3 88.2,215.5 52.0,222.1" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,212.7 88.2,204.9 124.4,186.3 160.5,179.8 196.7,165.0 232.9,147.1 269.1,145.3 305.3,132.7 341.5,119.5 377.6,108.4 413.8,86.1 450.0,57.7 450.0,159.6 413.8,163.1 377.6,167.5 341.5,168.1 305.3,174.3 269.1,177.4 232.9,179.3 196.7,184.4 160.5,190.9 124.4,195.1 88.2,216.1 52.0,223.5" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,207.8 88.2,201.6 124.4,186.7 160.5,193.5 196.7,175.2 232.9,147.1 269.1,108.1 305.3,132.5 341.5,125.7 377.6,113.6 413.8,99.9 450.0,95.1 450.0,139.4 413.8,146.5 377.6,155.4 341.5,162.2 305.3,172.2 269.1,177.9 232.9,185.1 196.7,198.9 160.5,202.7 124.4,206.6 88.2,210.6 52.0,221.1" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,219.4 88.2,212.6 269.1,177.3 305.3,174.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,219.3 88.2,212.9 124.4,189.9 160.5,184.5 196.7,176.8 232.9,170.8 269.1,166.9 305.3,162.4 341.5,155.9 377.6,152.0 413.8,146.9 450.0,143.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.3 88.2,207.2 124.4,202.7 160.5,199.1 196.7,188.0 232.9,180.1 269.1,171.7 305.3,165.0 341.5,156.8 377.6,150.4 413.8,141.4 450.0,135.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 214 ns (0.93×) | 190 ns (0.83×) | 230 ns |
| D38 | 362 ns (0.71×) | 351 ns (0.69×) | 509 ns |
| D57 | · | 1.78 µs (2.6×) | 692 ns |
| D76 | · | 2.61 µs (2.8×) | 922 ns |
| D115 | · | 4.51 µs (2.2×) | 2.09 µs |
| D153 | · | 6.56 µs (1.9×) | 3.49 µs |
| D230 | 3.67 µs (0.59×) | 9.06 µs (1.5×) | 6.2 µs |
| D307 | 4.77 µs (0.48×) | 11.3 µs (1.1×) | 10 µs |
| D462 | · | 17 µs (0.87×) | 19.7 µs |
| D616 | · | 20.6 µs (0.66×) | 31.1 µs |
| D924 | · | 31 µs (0.55×) | 56 µs |
| D1232 | · | 46.5 µs (0.5×) | 93.8 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,220.0 88.2,210.8 269.1,177.0 305.3,174.0 305.3,189.7 269.1,189.5 88.2,215.7 52.0,222.5" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,218.5 88.2,210.8 124.4,189.0 160.5,160.6 196.7,149.7 232.9,145.3 269.1,149.1 305.3,138.3 341.5,124.4 377.6,66.3 413.8,64.2 450.0,60.1 450.0,158.8 413.8,163.4 377.6,168.9 341.5,167.6 305.3,174.9 269.1,176.9 232.9,179.7 196.7,186.9 160.5,193.1 124.4,198.3 88.2,216.8 52.0,223.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,215.3 88.2,206.4 124.4,198.9 160.5,196.7 196.7,177.6 232.9,166.0 269.1,152.0 305.3,136.2 341.5,125.1 377.6,112.8 413.8,101.6 450.0,68.7 450.0,138.8 413.8,149.8 377.6,159.5 341.5,163.0 305.3,173.5 269.1,178.6 232.9,189.0 196.7,200.2 160.5,204.2 124.4,210.1 88.2,210.7 52.0,221.4" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,220.9 88.2,213.2 269.1,179.3 305.3,175.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,222.6 88.2,213.6 124.4,189.9 160.5,184.3 196.7,176.3 232.9,170.8 269.1,166.1 305.3,162.9 341.5,156.9 377.6,154.1 413.8,148.1 450.0,142.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.8 88.2,208.2 124.4,203.7 160.5,199.5 196.7,187.6 232.9,180.0 269.1,171.7 305.3,164.6 341.5,154.8 377.6,148.1 413.8,139.5 450.0,131.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 205 ns (0.88×) | 203 ns (0.87×) | 233 ns |
| D38 | 345 ns (0.67×) | 338 ns (0.66×) | 512 ns |
| D57 | · | 1.84 µs (2.5×) | 742 ns |
| D76 | 1.06 µs (1×) | 2.64 µs (2.6×) | 1.02 µs |
| D115 | · | 5.25 µs (2.6×) | 2 µs |
| D153 | 2.62 µs (0.61×) | 7.88 µs (1.8×) | 4.31 µs |
| D230 | 3.44 µs (0.45×) | 11.3 µs (1.5×) | 7.67 µs |
| D307 | 4.46 µs (0.36×) | 14.4 µs (1.2×) | 12.4 µs |
| D462 | · | 21.4 µs (1.1×) | 20.2 µs |
| D616 | · | 29 µs (0.96×) | 30.2 µs |
| D924 | · | 41.6 µs (0.69×) | 60.2 µs |
| D1232 | · | 51.3 µs (0.6×) | 86 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,218.1 88.2,211.4 160.5,197.2 232.9,182.2 269.1,177.6 305.3,174.6 305.3,192.7 269.1,193.3 232.9,193.0 160.5,206.0 88.2,218.6 52.0,223.1" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,217.8 88.2,206.2 124.4,169.8 160.5,178.1 196.7,156.7 232.9,145.9 269.1,137.7 305.3,125.7 341.5,112.1 377.6,101.5 413.8,61.8 450.0,60.4 450.0,160.3 413.8,163.1 377.6,168.2 341.5,170.3 305.3,175.6 269.1,179.9 232.9,181.7 196.7,189.7 160.5,194.6 124.4,201.1 88.2,218.7 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,215.6 88.2,204.9 124.4,194.6 160.5,191.6 196.7,170.2 232.9,149.9 269.1,142.9 305.3,126.7 341.5,115.4 377.6,103.9 413.8,87.2 450.0,61.3 450.0,140.5 413.8,147.1 377.6,156.6 341.5,163.1 305.3,174.5 269.1,179.3 232.9,185.8 196.7,198.6 160.5,206.3 124.4,212.2 88.2,212.8 52.0,222.2" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.5 88.2,213.9 160.5,197.5 232.9,184.2 269.1,180.3 305.3,176.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.6 88.2,214.2 124.4,189.4 160.5,184.1 196.7,174.1 232.9,168.2 269.1,162.9 305.3,159.4 341.5,153.5 377.6,149.1 413.8,143.8 450.0,140.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.6 88.2,208.1 124.4,202.7 160.5,198.0 196.7,188.2 232.9,177.0 269.1,168.5 305.3,161.6 341.5,154.4 377.6,148.5 413.8,138.4 450.0,133.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body:arithmetic -->
