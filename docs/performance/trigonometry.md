# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.16 ns | 2.11 µs | 3.21 µs | 3.54 µs | 3.82 µs |
| D38 | 1.29 µs | 4.67 µs | 4.05 µs | 8.58 µs | 9.6 µs |
| D57 | 1.56 µs | 6.35 µs | 6.83 µs | 10 µs | 14.2 µs |
| D76 | 1.17 µs | 7.19 µs | 10.6 µs | 15.6 µs | 18.9 µs |
| D115 | 1.59 µs | 8.25 µs | 20.1 µs | 34.4 µs | 40.1 µs |
| D153 | 1.61 µs | 7.08 µs | 15.5 µs | 44.7 µs | 54.2 µs |
| D230 | 1.22 µs | 21.1 µs | 45.7 µs | 70.4 µs | 132 µs |
| D307 | 1.25 µs | 22 µs | 60.4 µs | 124 µs | 195 µs |
| D462 | 1.55 µs | 29 µs | 74.6 µs | 244 µs | 381 µs |
| D616 | 1.56 µs | 66.1 µs | 208 µs | 277 µs | 558 µs |
| D924 | 1.65 µs | 127 µs | 451 µs | 747 µs | 1.68 ms |
| D1232 | 2.07 µs | 217 µs | 663 µs | 1 ms | 3.28 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.4 88.2,121.1 124.4,118.8 160.5,122.4 196.7,118.5 232.9,118.4 269.1,121.8 305.3,121.5 341.5,118.9 377.6,118.7 413.8,118.0 450.0,115.2 450.0,23.8 413.8,32.2 377.6,45.8 341.5,50.5 305.3,58.9 269.1,63.7 232.9,74.7 196.7,78.5 160.5,87.8 124.4,91.3 88.2,96.2 52.0,107.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.4 88.2,121.1 124.4,118.8 160.5,122.4 196.7,118.5 232.9,118.4 269.1,121.8 305.3,121.5 341.5,118.9 377.6,118.7 413.8,118.0 450.0,115.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.0 88.2,105.2 124.4,101.4 160.5,99.8 196.7,98.1 232.9,100.0 269.1,86.5 305.3,85.9 341.5,82.5 377.6,72.3 413.8,64.2 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,106.9 124.4,100.4 160.5,94.9 196.7,87.1 232.9,90.3 269.1,76.8 305.3,73.4 341.5,70.8 377.6,58.1 413.8,48.4 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.6 88.2,97.6 124.4,95.7 160.5,90.2 196.7,80.4 232.9,77.1 269.1,71.5 305.3,64.5 341.5,56.1 377.6,54.5 413.8,42.2 450.0,38.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,96.2 124.4,91.3 160.5,87.8 196.7,78.5 232.9,74.7 269.1,63.7 305.3,58.9 341.5,50.5 377.6,45.8 413.8,32.2 450.0,23.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 µs | 3.76 µs | 5.85 µs | 6.45 µs | 6.9 µs |
| D38 | 3.2 µs | 5.81 µs | 7.1 µs | 8.76 µs | 9.7 µs |
| D57 | 3.73 µs | 4.77 µs | 4.22 µs | 5.65 µs | 7.77 µs |
| D76 | 2.7 µs | 5.04 µs | 6.32 µs | 8.5 µs | 9.54 µs |
| D115 | 6.8 µs | 8.67 µs | 13.7 µs | 18.2 µs | 19.8 µs |
| D153 | 6.71 µs | 7.64 µs | 8.97 µs | 23.9 µs | 26.6 µs |
| D230 | 7.11 µs | 16.8 µs | 28.4 µs | 41.8 µs | 74.4 µs |
| D307 | 10.6 µs | 22.6 µs | 48.1 µs | 86 µs | 142 µs |
| D462 | 12.9 µs | 23.1 µs | 48 µs | 164 µs | 268 µs |
| D616 | 22.2 µs | 75.4 µs | 168 µs | 203 µs | 442 µs |
| D924 | 32.8 µs | 157 µs | 403 µs | 638 µs | 1.45 ms |
| D1232 | 45 µs | 274 µs | 682 µs | 975 µs | 2.91 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.2 88.2,184.7 124.4,181.4 160.5,188.4 196.7,168.4 232.9,168.7 269.1,167.4 305.3,158.8 341.5,154.5 377.6,142.7 413.8,134.2 450.0,127.3 450.0,36.8 413.8,52.0 377.6,77.7 341.5,88.6 305.3,102.4 269.1,116.4 232.9,138.8 196.7,145.2 160.5,161.0 124.4,165.5 88.2,160.7 52.0,168.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.2 88.2,184.7 124.4,181.4 160.5,188.4 196.7,168.4 232.9,168.7 269.1,167.4 305.3,158.8 341.5,154.5 377.6,142.7 413.8,134.2 450.0,127.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.3 88.2,171.8 124.4,176.1 160.5,174.9 196.7,163.1 232.9,165.8 269.1,148.7 305.3,142.3 341.5,141.8 377.6,116.1 413.8,100.2 450.0,88.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.7 88.2,167.4 124.4,178.7 160.5,170.0 196.7,153.2 232.9,162.4 269.1,137.4 305.3,125.9 341.5,125.9 377.6,98.7 413.8,79.7 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.5 88.2,162.9 124.4,172.4 160.5,163.5 196.7,147.0 232.9,141.1 269.1,128.9 305.3,113.3 341.5,99.2 377.6,94.6 413.8,69.8 450.0,60.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.1 88.2,160.7 124.4,165.5 160.5,161.0 196.7,145.2 232.9,138.8 269.1,116.4 305.3,102.4 341.5,88.6 377.6,77.7 413.8,52.0 450.0,36.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.55 ns | 2.1 µs | 3.2 µs | 3.51 µs | 3.8 µs |
| D38 | 1.21 µs | 4.63 µs | 4.01 µs | 8.54 µs | 9.59 µs |
| D57 | 1.45 µs | 6.33 µs | 6.77 µs | 9.97 µs | 14.2 µs |
| D76 | 1.06 µs | 7.19 µs | 10.6 µs | 15.6 µs | 18.9 µs |
| D115 | 1.44 µs | 8.22 µs | 20.1 µs | 34.4 µs | 40.4 µs |
| D153 | 1.48 µs | 7.05 µs | 15.5 µs | 44.7 µs | 54 µs |
| D230 | 1.14 µs | 20.8 µs | 45.4 µs | 70.1 µs | 131 µs |
| D307 | 1.18 µs | 23.2 µs | 60.6 µs | 124 µs | 192 µs |
| D462 | 1.42 µs | 28.9 µs | 75.3 µs | 246 µs | 382 µs |
| D616 | 1.43 µs | 67.7 µs | 208 µs | 278 µs | 558 µs |
| D924 | 1.54 µs | 128 µs | 450 µs | 743 µs | 1.68 ms |
| D1232 | 1.93 µs | 218 µs | 663 µs | 1.11 ms | 3.28 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.3 88.2,121.9 124.4,119.7 160.5,123.6 196.7,119.7 232.9,119.4 269.1,122.7 305.3,122.3 341.5,119.9 377.6,119.9 413.8,118.9 450.0,116.1 450.0,23.8 413.8,32.2 377.6,45.8 341.5,50.5 305.3,59.1 269.1,63.8 232.9,74.8 196.7,78.4 160.5,87.8 124.4,91.3 88.2,96.2 52.0,107.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.3 88.2,121.9 124.4,119.7 160.5,123.6 196.7,119.7 232.9,119.4 269.1,122.7 305.3,122.3 341.5,119.9 377.6,119.9 413.8,118.9 450.0,116.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.1 88.2,105.3 124.4,101.4 160.5,99.8 196.7,98.1 232.9,100.1 269.1,86.6 305.3,85.3 341.5,82.6 377.6,72.0 413.8,64.1 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,107.1 124.4,100.5 160.5,95.0 196.7,87.0 232.9,90.3 269.1,77.0 305.3,73.4 341.5,70.7 377.6,58.1 413.8,48.5 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,97.7 124.4,95.8 160.5,90.2 196.7,80.4 232.9,77.1 269.1,71.5 305.3,64.5 341.5,56.0 377.6,54.5 413.8,42.3 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,96.2 124.4,91.3 160.5,87.8 196.7,78.4 232.9,74.8 269.1,63.8 305.3,59.1 341.5,50.5 377.6,45.8 413.8,32.2 450.0,23.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.38 ns | 1.81 µs | 2.51 µs | 2.84 µs | 2.99 µs |
| D38 | 4.04 ns | 2.52 µs | 3.09 µs | 3.91 µs | 4.26 µs |
| D57 | 2.81 ns | 5.84 µs | 6.06 µs | 8.23 µs | 11.8 µs |
| D76 | 2.42 ns | 6.78 µs | 9.33 µs | 12.8 µs | 14.1 µs |
| D115 | 8.18 ns | 12.8 µs | 20.4 µs | 26.3 µs | 31.8 µs |
| D153 | 11 ns | 11.8 µs | 14 µs | 36.1 µs | 38.8 µs |
| D230 | 13 ns | 25.5 µs | 44.4 µs | 62.9 µs | 102 µs |
| D307 | 22 ns | 32.3 µs | 68.2 µs | 115 µs | 178 µs |
| D462 | 40.9 ns | 35.9 µs | 67.4 µs | 200 µs | 321 µs |
| D616 | 43.8 ns | 109 µs | 224 µs | 254 µs | 525 µs |
| D924 | 29.2 ns | 229 µs | 528 µs | 741 µs | 1.6 ms |
| D1232 | 72.8 ns | 384 µs | 853 µs | 1.03 ms | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.9 88.2,192.7 124.4,197.2 160.5,199.0 196.7,183.9 232.9,180.2 269.1,178.2 305.3,171.6 341.5,164.0 377.6,163.1 413.8,168.1 450.0,156.8 450.0,25.5 413.8,32.7 377.6,46.6 341.5,52.7 305.3,60.0 269.1,66.9 232.9,78.9 196.7,81.4 160.5,91.5 124.4,93.7 88.2,106.3 52.0,110.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.9 88.2,192.7 124.4,197.2 160.5,199.0 196.7,183.9 232.9,180.2 269.1,178.2 305.3,171.6 341.5,164.0 377.6,163.1 413.8,168.1 450.0,156.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,112.8 124.4,102.4 160.5,100.5 196.7,92.6 232.9,93.6 269.1,84.1 305.3,81.2 341.5,79.9 377.6,66.1 413.8,56.9 450.0,50.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,110.3 124.4,101.9 160.5,96.6 196.7,86.9 232.9,91.5 269.1,77.2 305.3,71.9 341.5,72.0 377.6,57.1 413.8,46.5 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,107.4 124.4,98.1 160.5,92.6 196.7,83.7 232.9,79.8 269.1,72.9 305.3,65.4 341.5,58.6 377.6,55.6 413.8,42.3 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,106.3 124.4,93.7 160.5,91.5 196.7,81.4 232.9,78.9 269.1,66.9 305.3,60.0 341.5,52.7 377.6,46.6 413.8,32.7 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.41 ns | 1.64 µs | 2.69 µs | 2.98 µs | 3.26 µs |
| D38 | 2.8 ns | 2.81 µs | 3.22 µs | 3.74 µs | 4.12 µs |
| D57 | 1.78 ns | 3.14 µs | 2.93 µs | 3.77 µs | 5.42 µs |
| D76 | 393 ns | 5.62 µs | 8.34 µs | 12.8 µs | 15.8 µs |
| D115 | 589 ns | 6.35 µs | 17.1 µs | 31.1 µs | 36.5 µs |
| D153 | 625 ns | 5.47 µs | 10.2 µs | 41.1 µs | 49.6 µs |
| D230 | 449 ns | 19.3 µs | 42.6 µs | 64.4 µs | 123 µs |
| D307 | 491 ns | 19.7 µs | 49.1 µs | 115 µs | 181 µs |
| D462 | 607 ns | 24.4 µs | 63 µs | 214 µs | 328 µs |
| D616 | 653 ns | 60.8 µs | 195 µs | 255 µs | 528 µs |
| D924 | 763 ns | 118 µs | 423 µs | 715 µs | 1.58 ms |
| D1232 | 1 µs | 202 µs | 626 µs | 971 µs | 3.15 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.8 88.2,197.2 124.4,202.9 160.5,135.9 196.7,130.9 232.9,130.1 269.1,134.2 305.3,133.1 341.5,130.5 377.6,129.6 413.8,127.6 450.0,124.3 450.0,24.3 413.8,32.9 377.6,46.5 341.5,52.4 305.3,59.8 269.1,64.5 232.9,75.8 196.7,79.6 160.5,90.1 124.4,103.3 88.2,106.7 52.0,109.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.8 88.2,197.2 124.4,202.9 160.5,135.9 196.7,130.9 232.9,130.1 269.1,134.2 305.3,133.1 341.5,130.5 377.6,129.6 413.8,127.6 450.0,124.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,111.4 124.4,110.1 160.5,102.9 196.7,101.3 232.9,103.2 269.1,87.6 305.3,87.3 341.5,84.6 377.6,73.3 413.8,65.1 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,109.8 124.4,110.9 160.5,98.0 196.7,89.1 232.9,95.5 269.1,77.7 305.3,76.0 341.5,72.9 377.6,58.9 413.8,49.2 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,107.9 124.4,107.8 160.5,92.6 196.7,81.6 232.9,78.2 269.1,72.6 305.3,65.4 341.5,57.7 377.6,55.5 413.8,42.7 450.0,38.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.6 88.2,106.7 124.4,103.3 160.5,90.1 196.7,79.6 232.9,75.8 269.1,64.5 305.3,59.8 341.5,52.4 377.6,46.5 413.8,32.9 450.0,24.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.81 µs | 2.86 µs | 3.19 µs | 3.46 µs |
| D38 | 4.04 ns | 2.85 µs | 3.57 µs | 4.37 µs | 4.87 µs |
| D57 | 541 ns | 6.18 µs | 5.86 µs | 8.03 µs | 11.5 µs |
| D76 | 321 ns | 6.68 µs | 8.77 µs | 12.1 µs | 14.4 µs |
| D115 | 1.15 µs | 12.1 µs | 20 µs | 28.3 µs | 30.6 µs |
| D153 | 1.08 µs | 10.5 µs | 13.3 µs | 37.2 µs | 43.1 µs |
| D230 | 1.02 µs | 25 µs | 43.9 µs | 70.3 µs | 127 µs |
| D307 | 1.62 µs | 34.2 µs | 80.9 µs | 148 µs | 249 µs |
| D462 | 2.03 µs | 35.8 µs | 79.8 µs | 293 µs | 485 µs |
| D616 | 3.76 µs | 124 µs | 294 µs | 358 µs | 811 µs |
| D924 | 5.01 µs | 264 µs | 716 µs | 1.17 ms | 2.69 ms |
| D1232 | 7.31 µs | 469 µs | 1.23 ms | 1.84 ms | 5.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.7 124.4,131.9 160.5,138.4 196.7,122.6 232.9,123.3 269.1,124.0 305.3,118.3 341.5,115.5 377.6,107.8 413.8,104.3 450.0,99.6 450.0,17.5 413.8,26.3 377.6,41.2 341.5,47.6 305.3,55.8 269.1,64.2 232.9,77.6 196.7,81.8 160.5,91.2 124.4,94.0 88.2,104.6 52.0,108.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.7 124.4,131.9 160.5,138.4 196.7,122.6 232.9,123.3 269.1,124.0 305.3,118.3 341.5,115.5 377.6,107.8 413.8,104.3 450.0,99.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,111.3 124.4,101.7 160.5,100.7 196.7,93.4 232.9,95.1 269.1,84.4 305.3,80.5 341.5,79.9 377.6,64.4 413.8,55.1 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,108.5 124.4,102.3 160.5,97.3 196.7,87.1 232.9,92.2 269.1,77.3 305.3,69.8 341.5,69.9 377.6,53.8 413.8,42.7 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,106.0 124.4,98.4 160.5,93.4 196.7,82.8 232.9,79.4 269.1,71.5 305.3,62.2 341.5,53.8 377.6,51.3 413.8,36.6 450.0,31.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,104.6 124.4,94.0 160.5,91.2 196.7,81.8 232.9,77.6 269.1,64.2 305.3,55.8 341.5,47.6 377.6,41.2 413.8,26.3 450.0,17.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.69 ns | 1.25 µs | 2.51 µs | 2.65 µs | 2.88 µs |
| D38 | 4.99 ns | 2.52 µs | 3.02 µs | 3.73 µs | 3.85 µs |
| D57 | 2.78 ns | 3.76 µs | 3.98 µs | 5.16 µs | 9.15 µs |
| D76 | 2.4 ns | 4.06 µs | 6.07 µs | 7.98 µs | 9.23 µs |
| D115 | 13.1 ns | 4.66 µs | 10.1 µs | 14.8 µs | 16.9 µs |
| D153 | 18.2 ns | 3.75 µs | 6.2 µs | 19.1 µs | 26.1 µs |
| D230 | 29.7 ns | 10.2 µs | 19.8 µs | 36.3 µs | 73.8 µs |
| D307 | 56.7 ns | 10.7 µs | 24.6 µs | 70 µs | 117 µs |
| D462 | 105 ns | 9.62 µs | 36.9 µs | 136 µs | 225 µs |
| D616 | 120 ns | 33.3 µs | 123 µs | 168 µs | 359 µs |
| D924 | 149 ns | 71.5 µs | 273 µs | 490 µs | 1.13 ms |
| D1232 | 367 ns | 129 µs | 430 µs | 725 µs | 2.32 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.8 88.2,190.1 124.4,197.3 160.5,199.1 196.7,178.1 232.9,174.0 269.1,167.9 305.3,159.9 341.5,152.3 377.6,150.6 413.8,147.9 450.0,136.7 450.0,28.1 413.8,37.0 377.6,51.3 341.5,57.1 305.3,65.2 269.1,70.9 232.9,83.8 196.7,89.2 160.5,96.7 124.4,96.8 88.2,107.6 52.0,111.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.8 88.2,190.1 124.4,197.3 160.5,199.1 196.7,178.1 232.9,174.0 269.1,167.9 305.3,159.9 341.5,152.3 377.6,150.6 413.8,147.9 450.0,136.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.5 88.2,112.8 124.4,107.8 160.5,106.9 196.7,105.2 232.9,107.9 269.1,95.5 305.3,94.8 341.5,96.2 377.6,80.8 413.8,71.3 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.9 88.2,110.6 124.4,107.2 160.5,101.9 196.7,95.6 232.9,101.6 269.1,87.3 305.3,84.5 341.5,79.5 377.6,64.5 413.8,54.7 450.0,49.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.2 88.2,107.9 124.4,103.9 160.5,98.5 196.7,90.9 232.9,87.7 269.1,79.7 305.3,71.6 341.5,63.3 377.6,60.7 413.8,47.4 450.0,42.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,107.6 124.4,96.8 160.5,96.7 196.7,89.2 232.9,83.8 269.1,70.9 305.3,65.2 341.5,57.1 377.6,51.3 413.8,37.0 450.0,28.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.74 ns | 1.87 µs | 3.31 µs | 3.55 µs | 3.76 µs |
| D38 | 4.04 ns | 3.3 µs | 3.92 µs | 4.84 µs | 4.87 µs |
| D57 | 3.17 ns | 5.95 µs | 5.9 µs | 7.44 µs | 10.3 µs |
| D76 | 2.97 ns | 6.32 µs | 8.16 µs | 11.2 µs | 12.3 µs |
| D115 | 10.9 ns | 12.2 µs | 12.7 µs | 23.8 µs | 24.5 µs |
| D153 | 18 ns | 5.65 µs | 9.85 µs | 25 µs | 31.7 µs |
| D230 | 29.9 ns | 14.6 µs | 24.6 µs | 43.4 µs | 85 µs |
| D307 | 58.9 ns | 14.9 µs | 51.3 µs | 80.8 µs | 125 µs |
| D462 | 105 ns | 15.5 µs | 45.6 µs | 160 µs | 237 µs |
| D616 | 118 ns | 40.1 µs | 135 µs | 184 µs | 354 µs |
| D924 | 161 ns | 82.5 µs | 291 µs | 481 µs | 1 ms |
| D1232 | 364 ns | 143 µs | 420 µs | 638 µs | 2.67 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.5 88.2,192.7 124.4,195.7 160.5,196.5 196.7,180.4 232.9,174.2 269.1,167.9 305.3,159.4 341.5,152.2 377.6,150.8 413.8,147.0 450.0,136.8 450.0,26.4 413.8,38.6 377.6,51.4 341.5,56.4 305.3,64.3 269.1,69.2 232.9,81.4 196.7,84.6 160.5,93.2 124.4,95.4 88.2,104.7 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,192.7 124.4,195.7 160.5,196.5 196.7,180.4 232.9,174.2 269.1,167.9 305.3,159.4 341.5,152.2 377.6,150.8 413.8,147.0 450.0,136.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.5 88.2,109.5 124.4,102.2 160.5,101.4 196.7,93.2 232.9,102.8 269.1,91.0 305.3,90.8 341.5,90.3 377.6,78.5 413.8,69.5 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.4 88.2,107.3 124.4,102.3 160.5,98.2 196.7,92.7 232.9,95.9 269.1,84.5 305.3,75.4 341.5,76.9 377.6,63.5 413.8,53.9 450.0,49.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.6 88.2,104.7 124.4,99.4 160.5,94.3 196.7,85.0 232.9,84.4 269.1,77.5 305.3,69.8 341.5,61.3 377.6,59.6 413.8,47.7 450.0,44.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,104.7 124.4,95.4 160.5,93.2 196.7,84.6 232.9,81.4 269.1,69.2 305.3,64.3 341.5,56.4 377.6,51.4 413.8,38.6 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.27 ns | 1.13 µs | 2.36 µs | 2.6 µs | 2.83 µs |
| D38 | 4.36 ns | 2.36 µs | 2.85 µs | 3.56 µs | 3.8 µs |
| D57 | 2.81 ns | 3.52 µs | 3.78 µs | 5.17 µs | 9.15 µs |
| D76 | 2.91 ns | 3.88 µs | 5.71 µs | 7.89 µs | 8.83 µs |
| D115 | 13 ns | 4.36 µs | 10.1 µs | 14.2 µs | 16.3 µs |
| D153 | 18.1 ns | 3.57 µs | 6.37 µs | 18.8 µs | 25.8 µs |
| D230 | 29.7 ns | 10.5 µs | 20.3 µs | 36.2 µs | 70.9 µs |
| D307 | 51.7 ns | 10.8 µs | 23.9 µs | 66 µs | 115 µs |
| D462 | 97.3 ns | 9.93 µs | 34.1 µs | 135 µs | 221 µs |
| D616 | 110 ns | 31.8 µs | 120 µs | 167 µs | 359 µs |
| D924 | 154 ns | 68.7 µs | 272 µs | 486 µs | 1.13 ms |
| D1232 | 352 ns | 127 µs | 432 µs | 709 µs | 2.31 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.3 88.2,191.7 124.4,197.2 160.5,196.8 196.7,178.2 232.9,174.0 269.1,167.9 305.3,161.0 341.5,153.2 377.6,151.6 413.8,147.5 450.0,137.2 450.0,28.2 413.8,37.0 377.6,51.3 341.5,57.3 305.3,65.4 269.1,71.4 232.9,83.9 196.7,89.6 160.5,97.3 124.4,96.8 88.2,107.7 52.0,111.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.3 88.2,191.7 124.4,197.2 160.5,196.8 196.7,178.2 232.9,174.0 269.1,167.9 305.3,161.0 341.5,153.2 377.6,151.6 413.8,147.5 450.0,137.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.7 88.2,113.6 124.4,108.7 160.5,107.4 196.7,106.0 232.9,108.5 269.1,95.2 305.3,94.7 341.5,95.8 377.6,81.4 413.8,71.8 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.6 88.2,111.3 124.4,107.8 160.5,102.7 196.7,95.6 232.9,101.3 269.1,86.9 305.3,84.9 341.5,80.5 377.6,64.9 413.8,54.7 450.0,49.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.4 88.2,108.5 124.4,103.9 160.5,98.7 196.7,91.3 232.9,87.9 269.1,79.8 305.3,72.3 341.5,63.4 377.6,60.8 413.8,47.5 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,107.7 124.4,96.8 160.5,97.3 196.7,89.6 232.9,83.9 269.1,71.4 305.3,65.4 341.5,57.3 377.6,51.3 413.8,37.0 450.0,28.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.12 ns | 1.88 µs | 3.32 µs | 3.55 µs | 3.77 µs |
| D38 | 4.04 ns | 3.32 µs | 3.93 µs | 4.89 µs | 4.88 µs |
| D57 | 12.2 ns | 5.96 µs | 5.94 µs | 7.44 µs | 10.3 µs |
| D76 | 10.2 ns | 6.32 µs | 8.2 µs | 11.2 µs | 12.3 µs |
| D115 | 11.3 ns | 12.2 µs | 12.7 µs | 23.5 µs | 24.5 µs |
| D153 | 17.4 ns | 5.68 µs | 9.71 µs | 25.2 µs | 31.4 µs |
| D230 | 29.2 ns | 14.7 µs | 24.6 µs | 43.5 µs | 84.9 µs |
| D307 | 50.5 ns | 15 µs | 50.1 µs | 81 µs | 126 µs |
| D462 | 99.9 ns | 15.5 µs | 46.1 µs | 160 µs | 237 µs |
| D616 | 112 ns | 39.9 µs | 135 µs | 187 µs | 353 µs |
| D924 | 171 ns | 82.3 µs | 291 µs | 482 µs | 1 ms |
| D1232 | 366 ns | 142 µs | 419 µs | 634 µs | 2.67 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,192.7 124.4,179.0 160.5,181.2 196.7,180.0 232.9,174.6 269.1,168.1 305.3,161.3 341.5,152.9 377.6,151.5 413.8,146.2 450.0,136.8 450.0,26.4 413.8,38.6 377.6,51.5 341.5,56.4 305.3,64.3 269.1,69.2 232.9,81.5 196.7,84.6 160.5,93.1 124.4,95.3 88.2,104.6 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,192.7 124.4,179.0 160.5,181.2 196.7,180.0 232.9,174.6 269.1,168.1 305.3,161.3 341.5,152.9 377.6,151.5 413.8,146.2 450.0,136.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.5 88.2,109.4 124.4,102.1 160.5,101.4 196.7,93.3 232.9,102.7 269.1,90.9 305.3,90.7 341.5,90.3 377.6,78.5 413.8,69.6 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.4 88.2,107.3 124.4,102.2 160.5,98.2 196.7,92.7 232.9,96.1 269.1,84.5 305.3,75.7 341.5,76.7 377.6,63.4 413.8,53.9 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.6 88.2,104.6 124.4,99.4 160.5,94.3 196.7,85.1 232.9,84.2 269.1,77.5 305.3,69.8 341.5,61.3 377.6,59.4 413.8,47.6 450.0,44.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,104.6 124.4,95.3 160.5,93.1 196.7,84.6 232.9,81.5 269.1,69.2 305.3,64.3 341.5,56.4 377.6,51.5 413.8,38.6 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.3 ns | 2.3 µs | 3.99 µs | 4.31 µs | 4.69 µs |
| D38 | 4.37 ns | 3.99 µs | 4.82 µs | 5.93 µs | 6.22 µs |
| D57 | 3.17 ns | 4.71 µs | 5.17 µs | 6.63 µs | 9.45 µs |
| D76 | 3.16 ns | 5.06 µs | 7.57 µs | 10.1 µs | 11.1 µs |
| D115 | 9.27 ns | 5.86 µs | 12.9 µs | 17.8 µs | 20.3 µs |
| D153 | 18.1 ns | 4.7 µs | 7.88 µs | 22.8 µs | 30 µs |
| D230 | 30.7 ns | 12.7 µs | 23.2 µs | 41.1 µs | 79.8 µs |
| D307 | 48.7 ns | 13.4 µs | 28.6 µs | 76 µs | 127 µs |
| D462 | 94.9 ns | 11.8 µs | 38.6 µs | 149 µs | 239 µs |
| D616 | 107 ns | 37.6 µs | 134 µs | 183 µs | 391 µs |
| D924 | 140 ns | 78.7 µs | 296 µs | 520 µs | 1.21 ms |
| D1232 | 360 ns | 143 µs | 462 µs | 742 µs | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.2 88.2,191.7 124.4,195.7 160.5,195.7 196.7,182.4 232.9,174.1 269.1,167.5 305.3,161.8 341.5,153.5 377.6,152.0 413.8,148.7 450.0,137.0 450.0,27.5 413.8,36.2 377.6,50.2 341.5,56.3 305.3,64.2 269.1,69.9 232.9,82.1 196.7,86.9 160.5,94.4 124.4,96.4 88.2,101.6 52.0,105.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.2 88.2,191.7 124.4,195.7 160.5,195.7 196.7,182.4 232.9,174.1 269.1,167.5 305.3,161.8 341.5,153.5 377.6,152.0 413.8,148.7 450.0,137.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.0 88.2,107.1 124.4,105.0 160.5,104.2 196.7,102.3 232.9,105.1 269.1,92.8 305.3,92.1 341.5,93.6 377.6,79.3 413.8,70.1 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,104.8 124.4,103.9 160.5,99.2 196.7,92.5 232.9,98.7 269.1,85.2 305.3,82.7 341.5,79.0 377.6,63.5 413.8,53.7 450.0,48.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,102.2 124.4,100.8 160.5,95.6 196.7,88.6 232.9,85.5 269.1,78.2 305.3,70.6 341.5,62.2 377.6,59.7 413.8,46.7 450.0,42.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.1 88.2,101.6 124.4,96.4 160.5,94.4 196.7,86.9 232.9,82.1 269.1,69.9 305.3,64.2 341.5,56.3 377.6,50.2 413.8,36.2 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.05 ns | 1.91 µs | 3.48 µs | 3.72 µs | 3.94 µs |
| D38 | 4.04 ns | 3.48 µs | 3.96 µs | 4.88 µs | 4.9 µs |
| D57 | 2.81 µs | 6.16 µs | 6.24 µs | 7.77 µs | 10.8 µs |
| D76 | 2.14 µs | 6.5 µs | 8.47 µs | 11.5 µs | 12.8 µs |
| D115 | 5.77 µs | 13.1 µs | 13.7 µs | 24.2 µs | 25.3 µs |
| D153 | 2.97 µs | 5.97 µs | 10.3 µs | 25.2 µs | 32 µs |
| D230 | 2.39 µs | 15.3 µs | 25.9 µs | 44.4 µs | 87.4 µs |
| D307 | 2.52 µs | 15.1 µs | 52 µs | 82.2 µs | 127 µs |
| D462 | 3.1 µs | 15.8 µs | 46.1 µs | 162 µs | 239 µs |
| D616 | 3.26 µs | 40.8 µs | 137 µs | 185 µs | 355 µs |
| D924 | 3.45 µs | 84.5 µs | 294 µs | 485 µs | 1.01 ms |
| D1232 | 4.49 µs | 145 µs | 425 µs | 643 µs | 2.68 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,196.2 88.2,192.7 124.4,111.4 160.5,114.9 196.7,102.5 232.9,110.8 269.1,113.5 305.3,112.8 341.5,110.3 377.6,109.6 413.8,108.9 450.0,105.6 450.0,26.3 413.8,38.4 377.6,51.4 341.5,56.3 305.3,64.2 269.1,68.8 232.9,81.3 196.7,84.2 160.5,92.7 124.4,94.8 88.2,104.6 52.0,107.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,196.2 88.2,192.7 124.4,111.4 160.5,114.9 196.7,102.5 232.9,110.8 269.1,113.5 305.3,112.8 341.5,110.3 377.6,109.6 413.8,108.9 450.0,105.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.3 88.2,108.8 124.4,101.7 160.5,101.1 196.7,92.4 232.9,102.1 269.1,90.4 305.3,90.6 341.5,90.0 377.6,78.3 413.8,69.2 450.0,62.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,107.2 124.4,101.6 160.5,97.8 196.7,91.8 232.9,95.3 269.1,83.9 305.3,75.3 341.5,76.8 377.6,63.3 413.8,53.7 450.0,49.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.0 88.2,104.6 124.4,98.8 160.5,94.0 196.7,84.7 232.9,84.3 269.1,77.2 305.3,69.6 341.5,61.2 377.6,59.5 413.8,47.6 450.0,44.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.3 88.2,104.6 124.4,94.8 160.5,92.7 196.7,84.2 232.9,81.3 269.1,68.8 305.3,64.2 341.5,56.3 377.6,51.4 413.8,38.4 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.54 ns | 140 ns | 159 ns | 159 ns | 161 ns |
| D38 | 4.36 ns | 155 ns | 160 ns | 199 ns | 183 ns |
| D57 | 211 ns | 335 ns | 272 ns | 300 ns | 407 ns |
| D76 | 130 ns | 330 ns | 357 ns | 445 ns | 438 ns |
| D115 | 484 ns | 561 ns | 791 ns | 912 ns | 753 ns |
| D153 | 451 ns | 407 ns | 429 ns | 966 ns | 893 ns |
| D230 | 429 ns | 918 ns | 1.11 µs | 1.27 µs | 1.98 µs |
| D307 | 704 ns | 1.04 µs | 1.54 µs | 2.19 µs | 2.9 µs |
| D462 | 908 ns | 952 ns | 1.25 µs | 3.11 µs | 3.99 µs |
| D616 | 1.16 µs | 2.06 µs | 2.97 µs | 2.83 µs | 4.84 µs |
| D924 | 1.51 µs | 2.82 µs | 4.96 µs | 6.07 µs | 11.4 µs |
| D1232 | 2.46 µs | 4.43 µs | 7.26 µs | 7.94 µs | 28.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.0 88.2,184.4 124.4,117.0 160.5,125.4 196.7,102.6 232.9,103.8 269.1,104.7 305.3,96.1 341.5,91.7 377.6,87.4 413.8,82.8 450.0,74.4 450.0,31.7 413.8,47.8 377.6,62.6 341.5,66.0 305.3,71.5 269.1,78.2 232.9,92.0 196.7,94.9 160.5,104.4 124.4,105.6 88.2,119.5 52.0,121.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.0 88.2,184.4 124.4,117.0 160.5,125.4 196.7,102.6 232.9,103.8 269.1,104.7 305.3,96.1 341.5,91.7 377.6,87.4 413.8,82.8 450.0,74.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.2 88.2,122.4 124.4,109.0 160.5,109.2 196.7,100.0 232.9,105.6 269.1,91.5 305.3,89.4 341.5,90.9 377.6,77.4 413.8,72.0 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.9 88.2,121.8 124.4,112.6 160.5,107.9 196.7,94.1 232.9,104.7 269.1,88.2 305.3,82.5 341.5,86.2 377.6,71.1 413.8,62.2 450.0,55.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.0 88.2,118.0 124.4,110.9 160.5,104.0 196.7,91.6 232.9,90.6 269.1,85.9 305.3,76.4 341.5,70.3 377.6,71.9 413.8,58.7 450.0,54.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.8 88.2,119.5 124.4,105.6 160.5,104.4 196.7,94.9 232.9,92.0 269.1,78.2 305.3,71.5 341.5,66.0 377.6,62.6 413.8,47.8 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.54 ns | 168 ns | 196 ns | 203 ns | 201 ns |
| D38 | 4.36 ns | 193 ns | 198 ns | 216 ns | 201 ns |
| D57 | 268 ns | 401 ns | 322 ns | 375 ns | 488 ns |
| D76 | 176 ns | 428 ns | 456 ns | 534 ns | 529 ns |
| D115 | 619 ns | 671 ns | 923 ns | 1.06 µs | 853 ns |
| D153 | 588 ns | 485 ns | 515 ns | 1.05 µs | 1.02 µs |
| D230 | 596 ns | 1.07 µs | 1.24 µs | 1.48 µs | 2.12 µs |
| D307 | 951 ns | 1.26 µs | 1.84 µs | 2.5 µs | 3.24 µs |
| D462 | 1.26 µs | 1.03 µs | 1.46 µs | 3.47 µs | 4.38 µs |
| D616 | 1.5 µs | 2.29 µs | 3.35 µs | 3.28 µs | 5.22 µs |
| D924 | 1.95 µs | 3.2 µs | 5.45 µs | 6.48 µs | 12 µs |
| D1232 | 2.99 µs | 4.87 µs | 7.73 µs | 8.29 µs | 29.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.0 88.2,184.4 124.4,112.9 160.5,120.1 196.7,98.3 232.9,99.2 269.1,99.0 305.3,90.9 341.5,86.0 377.6,82.9 413.8,78.4 450.0,71.0 450.0,31.2 413.8,46.8 377.6,61.3 341.5,64.3 305.3,69.6 269.1,77.0 232.9,89.6 196.7,92.8 160.5,101.1 124.4,102.5 88.2,117.8 52.0,117.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.0 88.2,184.4 124.4,112.9 160.5,120.1 196.7,98.3 232.9,99.2 269.1,99.0 305.3,90.9 341.5,86.0 377.6,82.9 413.8,78.4 450.0,71.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.0 88.2,118.6 124.4,105.9 160.5,104.8 196.7,96.9 232.9,102.6 269.1,88.8 305.3,85.9 341.5,89.5 377.6,75.6 413.8,69.8 450.0,62.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.3 88.2,118.2 124.4,109.7 160.5,103.6 196.7,91.4 232.9,101.5 269.1,86.3 305.3,79.4 341.5,83.5 377.6,69.0 413.8,60.6 450.0,54.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.7 88.2,116.6 124.4,107.0 160.5,100.9 196.7,88.9 232.9,89.1 269.1,83.2 305.3,74.0 341.5,68.4 377.6,69.4 413.8,57.5 450.0,53.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.9 88.2,117.8 124.4,102.5 160.5,101.1 196.7,92.8 232.9,89.6 269.1,77.0 305.3,69.6 341.5,64.3 377.6,61.3 413.8,46.8 450.0,31.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
