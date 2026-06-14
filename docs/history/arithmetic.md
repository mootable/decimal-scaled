# History — Arithmetic

How the arithmetic operations have moved release over release. See the
[History overview](../history.md) for the time units, the width reference map, and how
these timings are measured.

<!-- BEGIN GENERATED:history:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 220 ns (0.88×) | 225 ns (0.9×) | 250 ns |
| D38 | 331 ns (0.61×) | 340 ns (0.63×) | 540 ns |
| D57 | · | 2.16 µs (2.8×) | 776 ns |
| D76 | 716 ns (0.81×) | 2.12 µs (2.4×) | 881 ns |
| D115 | · | 4.8 µs (1.9×) | 2.49 µs |
| D153 | 1.69 µs (0.41×) | 8.03 µs (1.9×) | 4.17 µs |
| D230 | 3.25 µs (0.43×) | 11.3 µs (1.5×) | 7.58 µs |
| D307 | 3.71 µs (0.34×) | 14.2 µs (1.3×) | 10.8 µs |
| D462 | · | 12.5 µs (1.1×) | 11.2 µs |
| D616 | · | 25.6 µs (0.9×) | 28.4 µs |
| D924 | · | 38.9 µs (0.72×) | 53.9 µs |
| D1232 | · | 60.9 µs (0.7×) | 87.1 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,209.3 88.2,208.4 160.5,197.6 232.9,182.9 269.1,177.9 305.3,174.2 305.3,190.4 269.1,189.6 232.9,191.6 160.5,203.7 88.2,218.0 52.0,223.4" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,216.9 88.2,210.7 124.4,182.6 160.5,181.6 196.7,160.4 232.9,149.1 269.1,135.6 305.3,128.8 341.5,89.4 377.6,75.2 413.8,86.7 450.0,60.7 450.0,158.8 413.8,163.4 377.6,169.2 341.5,178.8 305.3,176.2 269.1,177.2 232.9,181.1 196.7,190.5 160.5,195.5 124.4,194.6 88.2,218.0 52.0,223.4" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,212.1 88.2,204.6 124.4,197.7 160.5,188.9 196.7,170.4 232.9,158.1 269.1,138.7 305.3,127.1 341.5,98.9 377.6,104.3 413.8,72.8 450.0,74.3 450.0,140.5 413.8,148.2 377.6,156.8 341.5,174.6 305.3,176.9 269.1,183.5 232.9,189.0 196.7,195.6 160.5,206.8 124.4,210.0 88.2,211.4 52.0,221.2" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,220.5 88.2,214.5 160.5,203.2 232.9,190.6 269.1,181.1 305.3,179.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,220.1 88.2,214.1 124.4,187.1 160.5,187.3 196.7,175.4 232.9,167.9 269.1,162.9 305.3,159.5 341.5,161.4 377.6,150.9 413.8,144.8 450.0,138.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.6 88.2,207.3 124.4,202.0 160.5,200.2 196.7,185.0 232.9,177.4 269.1,168.7 305.3,163.5 341.5,163.0 377.6,149.4 413.8,140.0 450.0,133.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 220 ns (0.92×) | 191 ns (0.8×) | 240 ns |
| D38 | 1.12 µs (0.85×) | 1.14 µs (0.86×) | 1.32 µs |
| D57 | · | 2.68 µs (3.3×) | 816 ns |
| D76 | · | 2.52 µs (3.1×) | 811 ns |
| D115 | · | 4.8 µs (2.2×) | 2.21 µs |
| D153 | · | 7.99 µs (2.3×) | 3.52 µs |
| D230 | 5.35 µs (0.83×) | 11.1 µs (1.7×) | 6.42 µs |
| D307 | 6.38 µs (0.69×) | 14.7 µs (1.6×) | 9.26 µs |
| D462 | · | 14.2 µs (1.5×) | 9.58 µs |
| D616 | · | 33.3 µs (1.1×) | 31.4 µs |
| D924 | · | 47.8 µs (0.84×) | 57.2 µs |
| D1232 | · | 73.6 µs (0.79×) | 93.6 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,218.6 88.2,194.8 269.1,172.1 305.3,168.8 305.3,180.8 269.1,181.4 88.2,198.0 52.0,225.1" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,219.8 88.2,192.4 124.4,170.7 160.5,180.7 196.7,162.1 232.9,144.7 269.1,136.8 305.3,134.7 341.5,75.6 377.6,64.5 413.8,70.9 450.0,85.6 450.0,154.7 413.8,157.4 377.6,161.7 341.5,175.7 305.3,172.2 269.1,173.5 232.9,176.9 196.7,183.1 160.5,190.5 124.4,188.5 88.2,197.3 52.0,224.2" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,215.0 88.2,191.5 124.4,198.3 160.5,197.3 196.7,176.2 232.9,165.3 269.1,142.0 305.3,112.0 341.5,131.7 377.6,113.6 413.8,70.4 450.0,63.0 450.0,142.6 413.8,147.6 377.6,156.6 341.5,172.3 305.3,173.6 269.1,178.0 232.9,188.7 196.7,193.4 160.5,207.3 124.4,207.3 88.2,196.1 52.0,221.8" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,220.5 88.2,196.7 269.1,173.8 305.3,171.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,222.5 88.2,196.4 124.4,183.9 160.5,184.8 196.7,175.4 232.9,167.9 269.1,163.1 305.3,159.0 341.5,159.5 377.6,147.1 413.8,141.8 450.0,135.5" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,219.2 88.2,194.3 124.4,201.3 160.5,201.4 196.7,186.7 232.9,180.0 269.1,171.1 305.3,165.8 341.5,165.3 377.6,148.0 413.8,139.2 450.0,132.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 260 ns (0.96×) | 230 ns (0.85×) | 271 ns |
| D38 | 360 ns (0.62×) | 371 ns (0.64×) | 580 ns |
| D57 | · | 2.16 µs (3×) | 731 ns |
| D76 | · | 2.02 µs (2.7×) | 746 ns |
| D115 | · | 4.02 µs (1.9×) | 2.09 µs |
| D153 | · | 6.62 µs (2×) | 3.38 µs |
| D230 | 4.27 µs (0.71×) | 9.02 µs (1.5×) | 6.05 µs |
| D307 | 5 µs (0.56×) | 11.2 µs (1.3×) | 8.89 µs |
| D462 | · | 10.7 µs (1.2×) | 8.76 µs |
| D616 | · | 24 µs (1×) | 23 µs |
| D924 | · | 32.8 µs (0.72×) | 45.3 µs |
| D1232 | · | 44.9 µs (0.62×) | 72.8 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,215.4 88.2,209.0 269.1,175.4 305.3,172.6 305.3,183.7 269.1,184.9 88.2,217.5 52.0,223.4" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,212.5 88.2,203.1 124.4,183.5 160.5,183.4 196.7,165.0 232.9,145.5 269.1,145.1 305.3,132.3 341.5,127.7 377.6,109.1 413.8,59.9 450.0,60.5 450.0,158.7 413.8,161.8 377.6,165.8 341.5,177.8 305.3,175.0 269.1,176.6 232.9,180.0 196.7,187.0 160.5,194.8 124.4,192.1 88.2,216.9 52.0,223.4" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,213.2 88.2,201.2 124.4,186.0 160.5,197.3 196.7,175.7 232.9,164.4 269.1,151.2 305.3,133.5 341.5,133.6 377.6,112.2 413.8,76.5 450.0,64.0 450.0,140.4 413.8,148.2 377.6,157.0 341.5,173.6 305.3,172.6 269.1,178.0 232.9,185.1 196.7,192.1 160.5,206.0 124.4,206.8 88.2,210.3 52.0,220.4" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,218.0 88.2,213.3 269.1,177.1 305.3,174.8" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,219.8 88.2,212.8 124.4,187.1 160.5,188.1 196.7,178.0 232.9,170.7 269.1,166.2 305.3,163.0 341.5,163.6 377.6,151.9 413.8,147.3 450.0,142.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,217.4 88.2,206.3 124.4,202.9 160.5,202.6 196.7,187.6 232.9,180.5 269.1,172.0 305.3,166.4 341.5,166.6 377.6,152.5 413.8,142.6 450.0,135.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 201 ns (0.89×) | 220 ns (0.98×) | 225 ns |
| D38 | 345 ns (0.65×) | 341 ns (0.64×) | 531 ns |
| D57 | · | 2.14 µs (3.1×) | 691 ns |
| D76 | · | 2.05 µs (2.8×) | 731 ns |
| D115 | · | 4.01 µs (1.9×) | 2.07 µs |
| D153 | · | 6.59 µs (1.9×) | 3.4 µs |
| D230 | 3.8 µs (0.61×) | 8.99 µs (1.4×) | 6.2 µs |
| D307 | 4.79 µs (0.55×) | 10.9 µs (1.2×) | 8.79 µs |
| D462 | · | 9.57 µs (1×) | 9.55 µs |
| D616 | · | 22.3 µs (0.83×) | 26.9 µs |
| D924 | · | 28.8 µs (0.55×) | 52.4 µs |
| D1232 | · | 44.7 µs (0.55×) | 81.9 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,219.2 88.2,211.7 269.1,176.9 305.3,174.1 305.3,187.7 269.1,190.0 88.2,216.9 52.0,222.6" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,219.8 88.2,212.1 124.4,185.7 160.5,165.6 196.7,169.2 232.9,159.2 269.1,148.3 305.3,141.5 341.5,81.8 377.6,111.3 413.8,98.4 450.0,55.8 450.0,158.6 413.8,168.3 377.6,169.4 341.5,179.7 305.3,174.6 269.1,176.6 232.9,181.4 196.7,188.5 160.5,195.5 124.4,193.7 88.2,215.9 52.0,221.9" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,217.4 88.2,206.3 124.4,201.2 160.5,199.5 196.7,177.3 232.9,165.9 269.1,148.8 305.3,138.9 341.5,133.0 377.6,89.0 413.8,78.2 450.0,65.0 450.0,140.7 413.8,152.4 377.6,156.6 341.5,172.9 305.3,173.8 269.1,182.6 232.9,188.0 196.7,194.6 160.5,207.3 124.4,209.0 88.2,212.5 52.0,221.9" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,221.8 88.2,213.9 269.1,178.8 305.3,175.4" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,220.5 88.2,214.1 124.4,187.2 160.5,187.8 196.7,178.0 232.9,170.8 269.1,166.2 305.3,163.4 341.5,165.3 377.6,153.0 413.8,149.2 450.0,142.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,220.1 88.2,207.6 124.4,203.7 160.5,202.9 196.7,187.7 232.9,180.5 269.1,171.7 305.3,166.6 341.5,165.3 377.6,150.2 413.8,140.4 450.0,133.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 215 ns (0.88×) | 210 ns (0.86×) | 245 ns |
| D38 | 321 ns (0.63×) | 330 ns (0.65×) | 510 ns |
| D57 | · | 2.19 µs (2.8×) | 786 ns |
| D76 | 856 ns (1×) | 2.11 µs (2.6×) | 822 ns |
| D115 | · | 4.87 µs (1.9×) | 2.5 µs |
| D153 | 2.21 µs (0.53×) | 8.17 µs (2×) | 4.17 µs |
| D230 | 3.43 µs (0.45×) | 11.4 µs (1.5×) | 7.58 µs |
| D307 | 4.23 µs (0.39×) | 14 µs (1.3×) | 10.8 µs |
| D462 | · | 12.6 µs (1.1×) | 11.3 µs |
| D616 | · | 29.6 µs (1.1×) | 27.5 µs |
| D924 | · | 38.6 µs (0.69×) | 55.7 µs |
| D1232 | · | 53.1 µs (0.64×) | 83.2 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polygon points="52.0,216.4 88.2,211.3 160.5,200.0 232.9,182.2 269.1,177.7 305.3,174.7 305.3,190.6 269.1,194.8 232.9,194.1 160.5,210.0 88.2,218.6 52.0,225.1" fill="var(--dusk-purple,#7A6A8E)" fill-opacity="0.12"/><polygon points="52.0,216.4 88.2,206.0 124.4,183.2 160.5,181.2 196.7,160.4 232.9,148.8 269.1,134.3 305.3,125.3 341.5,120.7 377.6,75.4 413.8,66.3 450.0,60.8 450.0,159.0 413.8,163.7 377.6,170.3 341.5,179.9 305.3,177.0 269.1,178.7 232.9,181.7 196.7,191.0 160.5,198.0 124.4,196.9 88.2,218.0 52.0,225.1" fill="var(--md-accent-fg-color)" fill-opacity="0.12"/><polygon points="52.0,214.5 88.2,205.3 124.4,195.7 160.5,195.0 196.7,170.8 232.9,158.0 269.1,143.0 305.3,99.8 341.5,123.0 377.6,104.6 413.8,68.1 450.0,68.1 450.0,140.2 413.8,148.3 377.6,157.5 341.5,174.4 305.3,173.6 269.1,183.9 232.9,190.2 196.7,193.6 160.5,209.0 124.4,211.0 88.2,211.4 52.0,221.9" fill="var(--md-primary-fg-color)" fill-opacity="0.12"/><polyline points="52.0,220.8 88.2,214.9 160.5,200.6 232.9,186.8 269.1,180.3 305.3,177.3" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.2 88.2,214.5 124.4,186.9 160.5,187.4 196.7,175.2 232.9,167.6 269.1,162.8 305.3,159.8 341.5,161.2 377.6,148.8 413.8,144.9 450.0,140.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,218.9 88.2,208.2 124.4,201.9 160.5,201.2 196.7,184.9 232.9,177.5 269.1,168.7 305.3,163.6 341.5,162.9 377.6,149.9 413.8,139.6 450.0,133.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body:arithmetic -->
