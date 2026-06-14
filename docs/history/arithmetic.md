# History — Arithmetic

How the arithmetic operations have moved release over release. See the
[History overview](../history.md) for the time units, the width reference map, and how
these timings are measured.

<!-- BEGIN GENERATED:history:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 206 ns (0.86×) | 210 ns (0.88×) | 240 ns |
| D38 | 331 ns (0.66×) | 321 ns (0.64×) | 501 ns |
| D57 | · | 1.82 µs (2.3×) | 782 ns |
| D76 | 896 ns (0.86×) | 2.66 µs (2.6×) | 1.04 µs |
| D115 | · | 3.85 µs (1.7×) | 2.29 µs |
| D153 | 1.74 µs (0.5×) | 8.09 µs (2.3×) | 3.52 µs |
| D230 | 3.27 µs (0.41×) | 11.6 µs (1.5×) | 7.98 µs |
| D307 | 3.97 µs (0.32×) | 14.6 µs (1.2×) | 12.3 µs |
| D462 | · | 21.6 µs (0.98×) | 22.1 µs |
| D616 | · | 28.7 µs (0.91×) | 31.5 µs |
| D924 | · | 46.3 µs (0.82×) | 56.4 µs |
| D1232 | · | 55.9 µs (0.59×) | 94.6 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,214.9 88.2,209.0 160.5,195.5 232.9,183.3 269.1,178.1 305.3,174.4 305.3,193.0 269.1,191.4 232.9,192.6 160.5,201.2 88.2,218.6 52.0,223.4" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,162.9 88.2,211.7 124.4,185.2 160.5,178.1 196.7,164.7 232.9,148.1 269.1,93.1 305.3,125.7 341.5,112.3 377.6,69.7 413.8,85.8 450.0,59.2 450.0,159.8 413.8,160.7 377.6,169.1 341.5,171.1 305.3,175.1 269.1,177.7 232.9,180.0 196.7,189.9 160.5,192.6 124.4,197.8 88.2,218.5 52.0,222.6" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,214.5 88.2,206.0 124.4,196.8 160.5,193.1 196.7,172.3 232.9,151.3 269.1,141.2 305.3,126.6 341.5,93.2 377.6,86.0 413.8,65.9 450.0,74.4 450.0,139.0 413.8,146.2 377.6,156.9 341.5,162.9 305.3,175.0 269.1,185.0 232.9,192.0 196.7,200.1 160.5,205.1 124.4,209.8 88.2,211.0 52.0,221.2" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.4 88.2,214.5 160.5,199.9 232.9,190.2 269.1,181.0 305.3,178.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.2 88.2,214.9 124.4,189.6 160.5,184.0 196.7,178.6 232.9,167.8 269.1,162.5 305.3,159.1 341.5,153.4 377.6,149.2 413.8,142.3 450.0,139.5" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.2 88.2,208.4 124.4,201.9 160.5,197.7 196.7,186.2 232.9,179.9 269.1,168.0 305.3,161.7 341.5,153.1 377.6,147.9 413.8,139.4 450.0,131.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 211 ns (0.78×) | 200 ns (0.74×) | 270 ns |
| D38 | 1.04 µs (0.84×) | 1.03 µs (0.83×) | 1.24 µs |
| D57 | · | 2.16 µs (2.7×) | 810 ns |
| D76 | · | 3.17 µs (3.2×) | 1 µs |
| D115 | · | 4.18 µs (1.9×) | 2.16 µs |
| D153 | · | 8.19 µs (2.3×) | 3.58 µs |
| D230 | 5.33 µs (0.76×) | 11.8 µs (1.7×) | 7 µs |
| D307 | 6.44 µs (0.6×) | 16.1 µs (1.5×) | 10.7 µs |
| D462 | · | 26 µs (1.4×) | 19 µs |
| D616 | · | 32.5 µs (1.1×) | 30 µs |
| D924 | · | 56.2 µs (1×) | 56 µs |
| D1232 | · | 74 µs (0.76×) | 97.6 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,219.1 88.2,195.8 269.1,171.8 305.3,156.4 305.3,180.4 269.1,182.5 88.2,199.2 52.0,222.6" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,219.8 88.2,193.4 124.4,184.3 160.5,177.4 196.7,165.3 232.9,152.7 269.1,142.0 305.3,133.6 341.5,120.0 377.6,73.2 413.8,94.1 450.0,58.3 450.0,155.9 413.8,157.3 377.6,163.5 341.5,166.0 305.3,171.5 269.1,173.9 232.9,175.6 196.7,188.0 160.5,187.0 124.4,192.6 88.2,199.2 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,214.1 88.2,193.0 124.4,196.7 160.5,192.1 196.7,176.5 232.9,147.3 269.1,149.3 305.3,134.2 341.5,124.3 377.6,112.3 413.8,77.1 450.0,66.7 450.0,139.5 413.8,147.6 377.6,157.7 341.5,162.6 305.3,173.8 269.1,181.3 232.9,189.3 196.7,199.2 160.5,202.9 124.4,206.3 88.2,197.3 52.0,221.2" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.1 88.2,197.7 269.1,173.9 305.3,171.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.9 88.2,197.9 124.4,187.1 160.5,181.4 196.7,177.4 232.9,167.6 269.1,162.3 305.3,157.7 341.5,150.7 377.6,147.4 413.8,139.4 450.0,135.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,217.5 88.2,195.2 124.4,201.4 160.5,198.3 196.7,187.1 232.9,179.7 269.1,169.9 305.3,163.7 341.5,155.3 377.6,148.6 413.8,139.5 450.0,131.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 240 ns (0.89×) | 221 ns (0.82×) | 270 ns |
| D38 | 371 ns (0.69×) | 351 ns (0.65×) | 541 ns |
| D57 | · | 1.77 µs (2.4×) | 748 ns |
| D76 | · | 2.57 µs (2.7×) | 962 ns |
| D115 | · | 3.46 µs (1.7×) | 2.03 µs |
| D153 | · | 6.8 µs (2×) | 3.4 µs |
| D230 | 4.12 µs (0.62×) | 9.3 µs (1.4×) | 6.65 µs |
| D307 | 5.02 µs (0.5×) | 11.6 µs (1.2×) | 10 µs |
| D462 | · | 18.3 µs (1.1×) | 17.3 µs |
| D616 | · | 23.5 µs (0.94×) | 25.1 µs |
| D924 | · | 37 µs (0.8×) | 46.3 µs |
| D1232 | · | 45.4 µs (0.58×) | 78.3 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,213.3 88.2,206.8 269.1,175.6 305.3,172.3 305.3,185.6 269.1,187.5 88.2,215.5 52.0,222.6" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,214.5 88.2,202.9 124.4,168.8 160.5,179.7 196.7,167.7 232.9,151.2 269.1,143.3 305.3,132.7 341.5,119.6 377.6,108.4 413.8,63.4 450.0,60.4 450.0,158.2 413.8,160.1 377.6,167.6 341.5,167.9 305.3,174.3 269.1,176.7 232.9,179.6 196.7,189.7 160.5,191.5 124.4,195.2 88.2,216.4 52.0,223.3" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,212.8 88.2,202.7 124.4,186.4 160.5,193.4 196.7,175.3 232.9,164.3 269.1,149.2 305.3,109.8 341.5,94.8 377.6,119.5 413.8,97.5 450.0,66.9 450.0,139.5 413.8,145.9 377.6,156.4 341.5,162.9 305.3,172.5 269.1,177.2 232.9,185.3 196.7,193.7 160.5,202.7 124.4,206.9 88.2,209.0 52.0,221.1" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,219.2 88.2,212.8 269.1,177.6 305.3,174.8" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,220.4 88.2,213.6 124.4,190.0 160.5,184.5 196.7,180.2 232.9,170.3 269.1,165.7 305.3,162.5 341.5,155.8 377.6,152.2 413.8,145.5 450.0,142.5" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,217.5 88.2,207.3 124.4,202.6 160.5,198.9 196.7,188.0 232.9,180.4 269.1,170.6 305.3,164.6 341.5,156.7 377.6,151.2 413.8,142.2 450.0,134.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 210 ns (0.86×) | 215 ns (0.88×) | 245 ns |
| D38 | 340 ns (0.68×) | 340 ns (0.68×) | 501 ns |
| D57 | · | 1.76 µs (2.5×) | 696 ns |
| D76 | · | 2.57 µs (2.8×) | 916 ns |
| D115 | · | 3.31 µs (1.7×) | 1.98 µs |
| D153 | · | 6.6 µs (1.9×) | 3.39 µs |
| D230 | 3.81 µs (0.57×) | 9.1 µs (1.4×) | 6.68 µs |
| D307 | 4.64 µs (0.47×) | 11.2 µs (1.1×) | 9.96 µs |
| D462 | · | 16.5 µs (0.88×) | 18.7 µs |
| D616 | · | 21.7 µs (0.73×) | 29.9 µs |
| D924 | · | 32.6 µs (0.56×) | 57.9 µs |
| D1232 | · | 41.2 µs (0.47×) | 87.3 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,219.2 88.2,210.6 269.1,177.8 305.3,174.8 305.3,188.5 269.1,191.9 88.2,216.9 52.0,222.6" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,219.8 88.2,209.7 124.4,189.0 160.5,183.2 196.7,173.7 232.9,158.4 269.1,146.8 305.3,138.2 341.5,124.4 377.6,62.2 413.8,97.1 450.0,59.4 450.0,160.1 413.8,162.9 377.6,167.6 341.5,170.2 305.3,174.6 269.1,177.8 232.9,179.4 196.7,190.8 160.5,192.7 124.4,198.1 88.2,217.4 52.0,223.3" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,218.0 88.2,206.5 124.4,199.9 160.5,195.9 196.7,178.4 232.9,166.0 269.1,149.9 305.3,136.2 341.5,124.5 377.6,113.1 413.8,69.7 450.0,85.3 450.0,138.9 413.8,153.7 377.6,160.1 341.5,165.4 305.3,176.9 269.1,181.9 232.9,186.9 196.7,196.3 160.5,204.0 124.4,209.3 88.2,212.1 52.0,221.2" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.2 88.2,214.1 269.1,178.8 305.3,175.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,220.8 88.2,214.1 124.4,190.1 160.5,184.5 196.7,180.8 232.9,170.7 269.1,166.0 305.3,163.0 341.5,157.4 377.6,153.4 413.8,147.4 450.0,144.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.9 88.2,208.4 124.4,203.6 160.5,199.6 196.7,188.3 232.9,180.5 269.1,170.6 305.3,164.7 341.5,155.5 377.6,148.7 413.8,139.0 450.0,133.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 190 ns (0.79×) | 196 ns (0.82×) | 240 ns |
| D38 | 320 ns (0.65×) | 301 ns (0.61×) | 491 ns |
| D57 | · | 1.79 µs (2.3×) | 775 ns |
| D76 | 1.03 µs (1.1×) | 2.65 µs (2.7×) | 982 ns |
| D115 | · | 3.78 µs (1.7×) | 2.29 µs |
| D153 | 2.32 µs (0.55×) | 8.08 µs (1.9×) | 4.23 µs |
| D230 | 3.5 µs (0.44×) | 11.8 µs (1.5×) | 7.99 µs |
| D307 | 4.58 µs (0.37×) | 14.6 µs (1.2×) | 12.2 µs |
| D462 | · | 21.8 µs (1×) | 21.7 µs |
| D616 | · | 28.9 µs (0.87×) | 33.3 µs |
| D924 | · | 45.2 µs (0.76×) | 59.5 µs |
| D1232 | · | 55.1 µs (0.57×) | 96.4 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,218.0 88.2,212.1 160.5,197.2 232.9,183.9 269.1,177.2 305.3,174.6 305.3,190.4 269.1,193.3 232.9,191.9 160.5,205.5 88.2,219.8 52.0,224.2" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,217.5 88.2,208.1 124.4,185.0 160.5,177.8 196.7,164.4 232.9,148.5 269.1,97.3 305.3,126.5 341.5,73.5 377.6,101.4 413.8,60.2 450.0,60.6 450.0,159.5 413.8,161.2 377.6,169.6 341.5,172.0 305.3,183.1 269.1,178.3 232.9,181.2 196.7,192.0 160.5,194.0 124.4,200.8 88.2,219.2 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,214.9 88.2,206.2 124.4,194.4 160.5,191.3 196.7,172.4 232.9,116.0 269.1,140.9 305.3,125.7 341.5,89.4 377.6,104.7 413.8,84.4 450.0,64.7 450.0,142.3 413.8,146.3 377.6,157.4 341.5,163.2 305.3,173.8 269.1,183.8 232.9,189.5 196.7,200.8 160.5,205.5 124.4,212.9 88.2,214.9 52.0,221.8" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,222.6 88.2,215.0 160.5,197.9 232.9,186.0 269.1,180.0 305.3,176.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,222.2 88.2,215.9 124.4,189.9 160.5,184.1 196.7,178.9 232.9,167.8 269.1,162.3 305.3,159.2 341.5,153.3 377.6,149.1 413.8,142.6 450.0,139.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.2 88.2,208.7 124.4,202.1 160.5,198.6 196.7,186.2 232.9,177.3 269.1,168.0 305.3,161.7 341.5,153.4 377.6,147.1 413.8,138.6 450.0,131.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body:arithmetic -->
