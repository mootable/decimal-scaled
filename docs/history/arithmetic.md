# History — Arithmetic

How the arithmetic operations have moved release over release. See the
[History overview](../history.md) for the time units, the width reference map, and how
these timings are measured.

<!-- BEGIN GENERATED:history:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 216 ns (0.9×) | 205 ns (0.85×) | 240 ns |
| D38 | 340 ns (0.69×) | 341 ns (0.69×) | 491 ns |
| D57 | · | 2.15 µs (2.8×) | 762 ns |
| D76 | 1.01 µs (0.99×) | 2.58 µs (2.6×) | 1.01 µs |
| D115 | · | 4.94 µs (1.9×) | 2.54 µs |
| D153 | 1.96 µs (0.45×) | 8.22 µs (1.9×) | 4.33 µs |
| D230 | 3.38 µs (0.41×) | 12 µs (1.5×) | 8.14 µs |
| D307 | 4.32 µs (0.38×) | 15.1 µs (1.3×) | 11.3 µs |
| D462 | · | 22.2 µs (1×) | 21.6 µs |
| D616 | · | 26 µs (0.93×) | 27.9 µs |
| D924 | · | 40.5 µs (0.69×) | 59 µs |
| D1232 | · | 53.8 µs (0.61×) | 88.4 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,212.1 88.2,209.1 160.5,194.2 232.9,178.4 269.1,177.3 305.3,152.8 305.3,188.7 269.1,193.3 232.9,190.8 160.5,199.2 88.2,219.1 52.0,223.4" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,216.4 88.2,210.6 124.4,182.9 160.5,179.5 196.7,146.4 232.9,148.2 269.1,135.6 305.3,127.6 341.5,114.1 377.6,102.7 413.8,61.7 450.0,62.9 450.0,159.9 413.8,164.0 377.6,169.3 341.5,170.2 305.3,174.1 269.1,177.2 232.9,179.6 196.7,187.7 160.5,191.3 124.4,193.8 88.2,219.1 52.0,222.6" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,215.4 88.2,206.8 124.4,197.7 160.5,193.3 196.7,170.0 232.9,149.2 269.1,141.1 305.3,125.1 341.5,113.9 377.6,105.2 413.8,91.4 450.0,64.7 450.0,139.8 413.8,146.8 377.6,158.0 341.5,162.9 305.3,173.1 269.1,182.5 232.9,189.3 196.7,195.4 160.5,205.1 124.4,209.3 88.2,211.7 52.0,221.2" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,220.7 88.2,214.1 160.5,198.2 232.9,188.5 269.1,180.5 305.3,176.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.5 88.2,214.1 124.4,187.1 160.5,184.4 196.7,175.0 232.9,167.5 269.1,162.0 305.3,158.7 341.5,153.0 377.6,150.7 413.8,144.2 450.0,140.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.2 88.2,208.7 124.4,202.3 160.5,198.2 196.7,184.7 232.9,176.9 269.1,167.7 305.3,162.9 341.5,153.4 377.6,149.7 413.8,138.7 450.0,132.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 201 ns (0.91×) | 190 ns (0.86×) | 220 ns |
| D38 | 1.14 µs (0.89×) | 1.12 µs (0.88×) | 1.28 µs |
| D57 | · | 2.62 µs (3.3×) | 792 ns |
| D76 | · | 3.1 µs (3.1×) | 1.01 µs |
| D115 | · | 4.92 µs (2.2×) | 2.26 µs |
| D153 | · | 8.31 µs (2.3×) | 3.6 µs |
| D230 | 5.15 µs (0.79×) | 12.1 µs (1.8×) | 6.54 µs |
| D307 | 6.58 µs (0.67×) | 16.2 µs (1.7×) | 9.81 µs |
| D462 | · | 24 µs (1.2×) | 19.6 µs |
| D616 | · | 33.2 µs (1.2×) | 27.2 µs |
| D924 | · | 52 µs (0.89×) | 58.4 µs |
| D1232 | · | 70.1 µs (0.79×) | 88.2 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,218.6 88.2,194.6 269.1,171.7 305.3,169.3 305.3,178.1 269.1,183.6 88.2,198.0 52.0,224.2" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,219.2 88.2,192.0 124.4,182.0 160.5,160.7 196.7,161.6 232.9,146.0 269.1,141.7 305.3,133.9 341.5,117.6 377.6,91.6 413.8,58.1 450.0,59.2 450.0,156.8 413.8,159.8 377.6,165.7 341.5,165.4 305.3,169.0 269.1,174.1 232.9,175.2 196.7,183.7 160.5,186.8 124.4,188.5 88.2,197.7 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,213.6 88.2,191.7 124.4,199.7 160.5,194.0 196.7,174.6 232.9,143.0 269.1,150.1 305.3,137.5 341.5,115.0 377.6,115.6 413.8,72.0 450.0,67.7 450.0,141.5 413.8,146.7 377.6,156.7 341.5,163.1 305.3,172.6 269.1,178.0 232.9,189.5 196.7,192.4 160.5,204.6 124.4,208.4 88.2,196.0 52.0,221.2" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.8 88.2,196.4 269.1,174.4 305.3,170.8" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,222.6 88.2,196.7 124.4,184.3 160.5,181.8 196.7,175.0 232.9,167.4 269.1,161.9 305.3,157.6 341.5,151.9 377.6,147.1 413.8,140.6 450.0,136.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,220.5 88.2,194.7 124.4,201.7 160.5,198.2 196.7,186.4 232.9,179.6 269.1,170.9 305.3,165.0 341.5,154.9 377.6,150.0 413.8,138.9 450.0,132.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 250 ns (1×) | 230 ns (0.92×) | 251 ns |
| D38 | 370 ns (0.71×) | 351 ns (0.67×) | 521 ns |
| D57 | · | 2.13 µs (2.9×) | 731 ns |
| D76 | · | 2.5 µs (2.8×) | 902 ns |
| D115 | · | 4.03 µs (1.9×) | 2.12 µs |
| D153 | · | 6.84 µs (1.9×) | 3.52 µs |
| D230 | 4.01 µs (0.65×) | 9.33 µs (1.5×) | 6.16 µs |
| D307 | 5.09 µs (0.54×) | 11.9 µs (1.3×) | 9.37 µs |
| D462 | · | 18.5 µs (1.1×) | 17.3 µs |
| D616 | · | 21.7 µs (0.89×) | 24.2 µs |
| D924 | · | 33 µs (0.67×) | 49.5 µs |
| D1232 | · | 41.6 µs (0.56×) | 74.6 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,214.5 88.2,208.1 269.1,175.3 305.3,172.4 305.3,184.3 269.1,188.7 88.2,216.4 52.0,223.3" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,215.4 88.2,204.4 124.4,184.6 160.5,180.5 196.7,164.9 232.9,153.6 269.1,143.7 305.3,134.1 341.5,119.6 377.6,109.2 413.8,97.1 450.0,62.2 450.0,159.5 413.8,161.4 377.6,169.0 341.5,168.4 305.3,173.4 269.1,175.9 232.9,178.8 196.7,186.8 160.5,190.8 124.4,192.1 88.2,216.4 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,215.5 88.2,201.4 124.4,186.8 160.5,194.0 196.7,175.4 232.9,115.5 269.1,150.1 305.3,136.7 341.5,120.3 377.6,114.9 413.8,99.6 450.0,63.4 450.0,139.0 413.8,146.2 377.6,156.1 341.5,161.7 305.3,172.6 269.1,178.6 232.9,184.6 196.7,191.9 160.5,204.0 124.4,206.8 88.2,210.0 52.0,221.9" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,218.6 88.2,212.9 269.1,178.0 305.3,174.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,219.8 88.2,213.6 124.4,187.3 160.5,185.0 196.7,177.9 232.9,170.2 269.1,165.7 305.3,162.1 341.5,155.7 377.6,153.4 413.8,147.2 450.0,143.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.5 88.2,207.9 124.4,202.9 160.5,199.8 196.7,187.3 232.9,179.9 269.1,171.7 305.3,165.6 341.5,156.6 377.6,151.7 413.8,141.3 450.0,135.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 226 ns (0.98×) | 210 ns (0.91×) | 230 ns |
| D38 | 345 ns (0.69×) | 351 ns (0.7×) | 500 ns |
| D57 | · | 2.19 µs (3.3×) | 671 ns |
| D76 | · | 2.54 µs (2.8×) | 906 ns |
| D115 | · | 4 µs (1.8×) | 2.21 µs |
| D153 | · | 6.88 µs (2×) | 3.45 µs |
| D230 | 3.54 µs (0.56×) | 9.31 µs (1.5×) | 6.36 µs |
| D307 | 4.83 µs (0.51×) | 12 µs (1.3×) | 9.41 µs |
| D462 | · | 17.9 µs (0.95×) | 18.9 µs |
| D616 | · | 19.6 µs (0.61×) | 32.2 µs |
| D924 | · | 30.3 µs (0.52×) | 58.6 µs |
| D1232 | · | 47.6 µs (0.51×) | 94.2 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,219.2 88.2,211.7 269.1,161.9 305.3,173.5 305.3,188.1 269.1,192.4 88.2,217.5 52.0,222.6" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,219.1 88.2,210.6 124.4,186.4 160.5,183.2 196.7,169.2 232.9,160.0 269.1,147.9 305.3,139.8 341.5,123.8 377.6,114.5 413.8,64.6 450.0,58.8 450.0,160.2 413.8,163.4 377.6,169.6 341.5,170.0 305.3,172.9 269.1,176.4 232.9,179.7 196.7,188.0 160.5,193.5 124.4,194.9 88.2,216.4 52.0,222.6" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,215.9 88.2,206.5 124.4,201.0 160.5,195.5 196.7,154.2 232.9,166.0 269.1,150.9 305.3,133.3 341.5,106.0 377.6,93.4 413.8,92.1 450.0,80.3 450.0,145.2 413.8,149.3 377.6,158.1 341.5,163.0 305.3,172.8 269.1,178.3 232.9,188.2 196.7,193.9 160.5,204.0 124.4,209.0 88.2,212.8 52.0,220.5" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,220.1 88.2,213.9 269.1,179.9 305.3,175.3" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.2 88.2,213.6 124.4,186.8 160.5,184.7 196.7,178.1 232.9,170.1 269.1,165.7 305.3,162.0 341.5,156.2 377.6,154.8 413.8,148.5 450.0,141.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.8 88.2,208.5 124.4,204.2 160.5,199.8 196.7,186.7 232.9,180.2 269.1,171.3 305.3,165.6 341.5,155.3 377.6,147.6 413.8,138.8 450.0,131.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 200 ns (0.83×) | 201 ns (0.83×) | 241 ns |
| D38 | 340 ns (0.72×) | 330 ns (0.7×) | 471 ns |
| D57 | · | 2.16 µs (2.9×) | 747 ns |
| D76 | 1.14 µs (1.2×) | 2.65 µs (2.8×) | 951 ns |
| D115 | · | 4.78 µs (1.9×) | 2.54 µs |
| D153 | 2.53 µs (0.59×) | 8.27 µs (1.9×) | 4.31 µs |
| D230 | 3.33 µs (0.41×) | 11.9 µs (1.4×) | 8.18 µs |
| D307 | 4.39 µs (0.39×) | 14.9 µs (1.3×) | 11.3 µs |
| D462 | · | 22.1 µs (1×) | 21.9 µs |
| D616 | · | 25.2 µs (0.84×) | 30.1 µs |
| D924 | · | 41.7 µs (0.68×) | 61.5 µs |
| D1232 | · | 52.6 µs (0.65×) | 81.4 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,214.5 88.2,211.3 160.5,196.3 232.9,182.7 269.1,177.2 305.3,174.3 305.3,192.6 269.1,194.6 232.9,194.7 160.5,206.8 88.2,218.5 52.0,225.1" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,218.0 88.2,206.8 124.4,182.6 160.5,179.2 196.7,150.3 232.9,149.7 269.1,136.0 305.3,119.7 341.5,111.3 377.6,103.2 413.8,58.7 450.0,76.2 450.0,159.5 413.8,164.0 377.6,173.0 341.5,171.0 305.3,174.9 269.1,178.6 232.9,181.7 196.7,189.2 160.5,195.8 124.4,198.5 88.2,218.5 52.0,225.0" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,216.4 88.2,206.5 124.4,194.9 160.5,191.6 196.7,170.5 232.9,157.5 269.1,142.3 305.3,94.1 341.5,97.5 377.6,95.0 413.8,75.3 450.0,62.9 450.0,139.7 413.8,146.5 377.6,157.9 341.5,163.4 305.3,173.4 269.1,179.5 232.9,190.2 196.7,195.4 160.5,206.8 124.4,211.0 88.2,214.5 52.0,222.6" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.9 88.2,214.1 160.5,196.5 232.9,184.7 269.1,180.8 305.3,176.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.8 88.2,214.5 124.4,187.0 160.5,184.1 196.7,175.5 232.9,167.5 269.1,162.2 305.3,158.9 341.5,153.1 377.6,151.2 413.8,143.8 450.0,140.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.1 88.2,209.3 124.4,202.6 160.5,199.1 196.7,184.7 232.9,177.0 269.1,167.6 305.3,162.9 341.5,153.2 377.6,148.5 413.8,138.1 450.0,134.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body:arithmetic -->
