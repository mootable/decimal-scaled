# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.47 ns | 2.12 µs | 2.86 µs | 3.48 µs | 4.13 µs |
| D38 | 918 ns | 4.69 µs | 4.06 µs | 8.07 µs | 7.29 µs |
| D57 | 1.26 µs | 5.15 µs | 4.74 µs | 10.5 µs | 11.6 µs |
| D76 | 1.61 µs | 5.83 µs | 6.81 µs | 14.4 µs | 15.2 µs |
| D115 | 1.56 µs | 8.84 µs | 12.4 µs | 32.8 µs | 45.9 µs |
| D153 | 958 ns | 10.6 µs | 19.2 µs | 45 µs | 53.7 µs |
| D230 | 1.49 µs | 21.1 µs | 42.4 µs | 71.3 µs | 123 µs |
| D307 | 1.44 µs | 18.9 µs | 54.5 µs | 125 µs | 182 µs |
| D462 | 1.23 µs | 42.7 µs | 92.8 µs | 240 µs | 350 µs |
| D616 | 1.18 µs | 77.5 µs | 218 µs | 426 µs | 558 µs |
| D924 | 1.78 µs | 92.2 µs | 449 µs | 869 µs | 1.52 ms |
| D1232 | 1.55 µs | 207 µs | 659 µs | 1.49 ms | 2.2 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.1 88.2,125.3 124.4,121.4 160.5,118.4 196.7,118.8 232.9,124.8 269.1,119.3 305.3,119.7 341.5,121.7 377.6,122.2 413.8,117.1 450.0,118.9 450.0,28.8 413.8,33.4 377.6,45.8 341.5,51.6 305.3,59.7 269.1,64.6 232.9,74.9 196.7,76.8 160.5,90.5 124.4,93.9 88.2,99.6 52.0,106.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.1 88.2,125.3 124.4,121.4 160.5,118.4 196.7,118.8 232.9,124.8 269.1,119.3 305.3,119.7 341.5,121.7 377.6,122.2 413.8,117.1 450.0,118.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.0 88.2,105.1 124.4,104.0 160.5,102.4 196.7,97.2 232.9,95.0 269.1,86.4 305.3,87.8 341.5,77.7 377.6,70.3 413.8,68.1 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,106.9 124.4,105.0 160.5,100.5 196.7,93.1 232.9,87.6 269.1,77.8 305.3,74.7 341.5,68.1 377.6,57.4 413.8,48.5 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.4 124.4,95.1 160.5,91.2 196.7,81.0 232.9,77.1 269.1,71.3 305.3,64.4 341.5,56.3 377.6,49.2 413.8,40.3 450.0,33.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,99.6 124.4,93.9 160.5,90.5 196.7,76.8 232.9,74.9 269.1,64.6 305.3,59.7 341.5,51.6 377.6,45.8 413.8,33.4 450.0,28.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.23 µs | 3.76 µs | 5.23 µs | 6.39 µs | 7.37 µs |
| D38 | 2.57 µs | 5.86 µs | 7.11 µs | 8.37 µs | 8.36 µs |
| D57 | 2.89 µs | 3.69 µs | 2.88 µs | 6.17 µs | 6.47 µs |
| D76 | 3.71 µs | 3.88 µs | 4.03 µs | 7.88 µs | 7.9 µs |
| D115 | 6.71 µs | 9.53 µs | 9.06 µs | 17.2 µs | 23.9 µs |
| D153 | 4.31 µs | 11.1 µs | 11.2 µs | 23.5 µs | 26.9 µs |
| D230 | 8.51 µs | 16 µs | 26.5 µs | 43.2 µs | 69.6 µs |
| D307 | 12.7 µs | 18.8 µs | 40.6 µs | 85.6 µs | 133 µs |
| D462 | 10.1 µs | 36.4 µs | 60.6 µs | 161 µs | 248 µs |
| D616 | 16.4 µs | 89.1 µs | 179 µs | 317 µs | 437 µs |
| D924 | 34.4 µs | 105 µs | 400 µs | 746 µs | 1.33 ms |
| D1232 | 36.7 µs | 259 µs | 673 µs | 1.43 ms | 1.97 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.6 88.2,189.5 124.4,187.0 160.5,181.6 196.7,168.7 232.9,178.3 269.1,163.5 305.3,154.8 341.5,159.7 377.6,149.2 413.8,133.2 450.0,131.8 450.0,45.3 413.8,53.9 377.6,78.0 341.5,90.2 305.3,103.8 269.1,117.9 232.9,138.5 196.7,141.0 160.5,165.1 124.4,169.5 88.2,163.9 52.0,166.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.6 88.2,189.5 124.4,187.0 160.5,181.6 196.7,168.7 232.9,178.3 269.1,163.5 305.3,154.8 341.5,159.7 377.6,149.2 413.8,133.2 450.0,131.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.2 88.2,171.6 124.4,181.7 160.5,180.6 196.7,161.1 232.9,157.8 269.1,149.8 305.3,146.3 341.5,131.9 377.6,112.5 413.8,109.0 450.0,89.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.1 88.2,167.4 124.4,187.0 160.5,179.7 196.7,162.1 232.9,157.5 269.1,138.8 305.3,129.6 341.5,120.9 377.6,97.4 413.8,79.9 450.0,68.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.7 88.2,163.9 124.4,170.5 160.5,165.2 196.7,148.3 232.9,141.4 269.1,128.2 305.3,113.4 341.5,99.6 377.6,85.0 413.8,66.4 450.0,52.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,166.6 88.2,163.9 124.4,169.5 160.5,165.1 196.7,141.0 232.9,138.5 269.1,117.9 305.3,103.8 341.5,90.2 377.6,78.0 413.8,53.9 450.0,45.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 2.1 µs | 2.83 µs | 3.46 µs | 4.27 µs |
| D38 | 823 ns | 4.62 µs | 4.01 µs | 8.03 µs | 7.25 µs |
| D57 | 1.15 µs | 5.13 µs | 4.73 µs | 10.5 µs | 11.6 µs |
| D76 | 1.44 µs | 5.81 µs | 6.77 µs | 14.3 µs | 15.1 µs |
| D115 | 1.45 µs | 8.81 µs | 12.3 µs | 32.8 µs | 44.7 µs |
| D153 | 975 ns | 10.6 µs | 19.1 µs | 45.5 µs | 54.7 µs |
| D230 | 1.36 µs | 20.4 µs | 42.4 µs | 70.1 µs | 124 µs |
| D307 | 1.33 µs | 18.3 µs | 54.2 µs | 124 µs | 182 µs |
| D462 | 1.18 µs | 42.5 µs | 94.2 µs | 239 µs | 350 µs |
| D616 | 1.06 µs | 78.9 µs | 216 µs | 428 µs | 557 µs |
| D924 | 1.66 µs | 96.5 µs | 448 µs | 873 µs | 1.52 ms |
| D1232 | 1.44 µs | 207 µs | 658 µs | 1.5 ms | 2.2 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,126.7 124.4,122.5 160.5,119.8 196.7,119.7 232.9,124.6 269.1,120.4 305.3,120.8 341.5,122.2 377.6,123.5 413.8,118.0 450.0,119.7 450.0,28.8 413.8,33.3 377.6,45.8 341.5,51.6 305.3,59.7 269.1,64.5 232.9,74.6 196.7,77.1 160.5,90.6 124.4,93.9 88.2,99.7 52.0,106.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,126.7 124.4,122.5 160.5,119.8 196.7,119.7 232.9,124.6 269.1,120.4 305.3,120.8 341.5,122.2 377.6,123.5 413.8,118.0 450.0,119.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.1 88.2,105.3 124.4,104.0 160.5,102.5 196.7,97.3 232.9,95.0 269.1,86.8 305.3,88.2 341.5,77.8 377.6,70.1 413.8,67.6 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,107.1 124.4,105.0 160.5,100.6 196.7,93.1 232.9,87.7 269.1,77.8 305.3,74.7 341.5,67.9 377.6,57.6 413.8,48.5 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,98.4 124.4,95.1 160.5,91.2 196.7,81.0 232.9,76.9 269.1,71.6 305.3,64.4 341.5,56.3 377.6,49.1 413.8,40.3 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.3 88.2,99.7 124.4,93.9 160.5,90.6 196.7,77.1 232.9,74.6 269.1,64.5 305.3,59.7 341.5,51.6 377.6,45.8 413.8,33.3 450.0,28.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.64 µs | 1.81 µs | 2.26 µs | 2.79 µs | 3.21 µs |
| D38 | 1.28 µs | 2.55 µs | 3.09 µs | 3.7 µs | 3.57 µs |
| D57 | 3.86 µs | 4.54 µs | 4.37 µs | 8.86 µs | 9.75 µs |
| D76 | 4.91 µs | 5.21 µs | 6.01 µs | 11.6 µs | 11.6 µs |
| D115 | 9.24 µs | 13.6 µs | 12.4 µs | 24.7 µs | 37.1 µs |
| D153 | 5.91 µs | 16 µs | 17 µs | 35.6 µs | 39.2 µs |
| D230 | 11.4 µs | 24.6 µs | 41.7 µs | 62.7 µs | 96.2 µs |
| D307 | 16.7 µs | 25.5 µs | 56.8 µs | 114 µs | 167 µs |
| D462 | 13.1 µs | 55.4 µs | 84.8 µs | 197 µs | 290 µs |
| D616 | 21.3 µs | 129 µs | 242 µs | 402 µs | 528 µs |
| D924 | 44.7 µs | 161 µs | 528 µs | 860 µs | 1.45 ms |
| D1232 | 49.1 µs | 363 µs | 858 µs | 1.61 ms | 2.01 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.2 88.2,204.7 124.4,180.7 160.5,175.5 196.7,161.7 232.9,171.4 269.1,157.1 305.3,148.8 341.5,154.2 377.6,143.6 413.8,127.5 450.0,125.4 450.0,44.9 413.8,51.9 377.6,73.9 341.5,86.9 305.3,98.8 269.1,110.8 232.9,130.3 196.7,131.5 160.5,156.7 124.4,160.6 88.2,182.4 52.0,184.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.2 88.2,204.7 124.4,180.7 160.5,175.5 196.7,161.7 232.9,171.4 269.1,157.1 305.3,148.8 341.5,154.2 377.6,143.6 413.8,127.5 450.0,125.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.1 88.2,189.7 124.4,177.1 160.5,174.2 196.7,153.3 232.9,149.7 269.1,140.4 305.3,139.7 341.5,122.8 377.6,104.4 413.8,99.7 450.0,82.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,192.3 88.2,185.5 124.4,178.0 160.5,171.1 196.7,155.3 232.9,148.5 269.1,129.0 305.3,122.3 341.5,113.6 377.6,90.8 413.8,73.9 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.7 88.2,181.6 124.4,162.6 160.5,156.7 196.7,140.4 232.9,132.4 269.1,120.2 305.3,107.1 341.5,95.3 377.6,79.8 413.8,63.3 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.7 88.2,182.4 124.4,160.6 160.5,156.7 196.7,131.5 232.9,130.3 269.1,110.8 305.3,98.8 341.5,86.9 377.6,73.9 413.8,51.9 450.0,44.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.4 µs | 1.64 µs | 2.39 µs | 2.94 µs | 3.55 µs |
| D38 | 1.14 µs | 2.68 µs | 3.14 µs | 3.47 µs | 2.95 µs |
| D57 | 1.5 µs | 2.39 µs | 1.97 µs | 4.06 µs | 4.29 µs |
| D76 | 5.98 µs | 4.58 µs | 5.26 µs | 11.8 µs | 12.7 µs |
| D115 | 5.95 µs | 6.81 µs | 10.3 µs | 30 µs | 40.3 µs |
| D153 | 4.09 µs | 8.33 µs | 13.7 µs | 40.8 µs | 49.9 µs |
| D230 | 5.67 µs | 17.2 µs | 38.5 µs | 64.9 µs | 115 µs |
| D307 | 5.57 µs | 15.4 µs | 45.7 µs | 116 µs | 168 µs |
| D462 | 2.46 µs | 34.5 µs | 77.8 µs | 209 µs | 300 µs |
| D616 | 4.52 µs | 69.6 µs | 202 µs | 396 µs | 526 µs |
| D924 | 6.22 µs | 87.1 µs | 422 µs | 823 µs | 1.4 ms |
| D1232 | 5.2 µs | 192 µs | 622 µs | 1.44 ms | 2.12 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.0 88.2,207.1 124.4,201.2 160.5,171.2 196.7,171.3 232.9,179.4 269.1,172.3 305.3,172.7 341.5,190.4 377.6,177.2 413.8,170.3 450.0,174.2 450.0,43.7 413.8,52.7 377.6,74.0 341.5,86.2 305.3,98.7 269.1,106.9 232.9,125.1 196.7,129.8 160.5,154.9 124.4,178.4 88.2,186.5 52.0,182.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.0 88.2,207.1 124.4,201.2 160.5,171.2 196.7,171.3 232.9,179.4 269.1,172.3 305.3,172.7 341.5,190.4 377.6,177.2 413.8,170.3 450.0,174.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,199.3 88.2,188.6 124.4,191.1 160.5,177.0 196.7,168.3 232.9,164.0 269.1,148.3 305.3,150.6 341.5,133.1 377.6,117.9 413.8,113.0 450.0,95.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.1 88.2,185.1 124.4,195.3 160.5,174.0 196.7,159.3 232.9,153.2 269.1,130.7 305.3,127.0 341.5,115.5 377.6,94.7 413.8,78.7 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.6 88.2,183.0 124.4,179.5 160.5,156.3 196.7,136.1 232.9,129.4 269.1,119.4 305.3,106.8 341.5,94.0 377.6,80.1 413.8,64.2 450.0,52.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.5 88.2,186.5 124.4,178.4 160.5,154.9 196.7,129.8 232.9,125.1 269.1,106.9 305.3,98.7 341.5,86.2 377.6,74.0 413.8,52.7 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.05 ns | 1.81 µs | 2.55 µs | 3.16 µs | 3.79 µs |
| D38 | 2.25 ns | 2.88 µs | 3.57 µs | 4.16 µs | 4.14 µs |
| D57 | 412 ns | 4.75 µs | 4.06 µs | 8.88 µs | 9.53 µs |
| D76 | 546 ns | 5.07 µs | 5.58 µs | 11.5 µs | 11.9 µs |
| D115 | 1.05 µs | 13.3 µs | 11.7 µs | 26.5 µs | 37.6 µs |
| D153 | 705 ns | 15.4 µs | 17 µs | 37 µs | 43.4 µs |
| D230 | 1.26 µs | 23.3 µs | 41.3 µs | 71.3 µs | 119 µs |
| D307 | 1.92 µs | 28.6 µs | 67.4 µs | 147 µs | 237 µs |
| D462 | 1.52 µs | 56.6 µs | 105 µs | 289 µs | 454 µs |
| D616 | 2.61 µs | 145 µs | 312 µs | 567 µs | 796 µs |
| D924 | 5.36 µs | 174 µs | 708 µs | 1.37 ms | 2.47 ms |
| D1232 | 5.58 µs | 442 µs | 1.21 ms | 2.65 ms | 3.73 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.6 88.2,199.9 124.4,135.3 160.5,131.8 196.7,123.7 232.9,128.6 269.1,121.4 305.3,116.2 341.5,119.1 377.6,112.4 413.8,103.4 450.0,102.9 450.0,22.2 413.8,27.4 377.6,41.4 341.5,48.4 305.3,56.5 269.1,65.0 232.9,77.5 196.7,79.3 160.5,93.6 124.4,96.3 88.2,106.7 52.0,107.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.6 88.2,199.9 124.4,135.3 160.5,131.8 196.7,123.7 232.9,128.6 269.1,121.4 305.3,116.2 341.5,119.1 377.6,112.4 413.8,103.4 450.0,102.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,111.1 124.4,105.0 160.5,104.1 196.7,92.2 232.9,90.3 269.1,85.2 305.3,82.7 341.5,74.2 377.6,62.5 413.8,60.3 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,108.5 124.4,106.9 160.5,103.0 196.7,93.8 232.9,89.1 269.1,78.1 305.3,72.0 341.5,66.6 377.6,53.0 413.8,42.9 450.0,36.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,106.6 124.4,97.2 160.5,94.0 196.7,83.6 232.9,79.5 269.1,71.3 305.3,62.4 341.5,54.0 377.6,45.6 413.8,34.7 450.0,26.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,106.7 124.4,96.3 160.5,93.6 196.7,79.3 232.9,77.5 269.1,65.0 305.3,56.5 341.5,48.4 377.6,41.4 413.8,27.4 450.0,22.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.25 µs | 1.25 µs | 2.02 µs | 2.62 µs | 3.12 µs |
| D38 | 1.11 µs | 2.51 µs | 3.02 µs | 3.47 µs | 2.85 µs |
| D57 | 2.65 µs | 3.11 µs | 2.66 µs | 5.57 µs | 7.45 µs |
| D76 | 3.2 µs | 3.34 µs | 3.58 µs | 7.19 µs | 7.59 µs |
| D115 | 3.2 µs | 5.12 µs | 6.57 µs | 13.8 µs | 19.5 µs |
| D153 | 1.95 µs | 5.84 µs | 7.45 µs | 19.1 µs | 26.2 µs |
| D230 | 3.04 µs | 10.1 µs | 19 µs | 37 µs | 70 µs |
| D307 | 3 µs | 9.05 µs | 22 µs | 69.7 µs | 112 µs |
| D462 | 1.66 µs | 15.1 µs | 44.7 µs | 135 µs | 210 µs |
| D616 | 2.7 µs | 38.1 µs | 130 µs | 259 µs | 359 µs |
| D924 | 3.55 µs | 49.6 µs | 270 µs | 571 µs | 1.05 ms |
| D1232 | 2.88 µs | 123 µs | 425 µs | 1.03 ms | 1.61 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,205.1 88.2,207.8 124.4,188.8 160.5,184.8 196.7,184.8 232.9,195.5 269.1,185.8 305.3,186.2 341.5,199.0 377.6,188.4 413.8,182.5 450.0,187.0 450.0,49.7 413.8,59.0 377.6,82.2 341.5,93.9 305.3,107.6 269.1,117.7 232.9,139.1 196.7,145.5 160.5,166.0 124.4,166.4 88.2,187.3 52.0,185.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,205.1 88.2,207.8 124.4,188.8 160.5,184.8 196.7,184.8 232.9,195.5 269.1,185.8 305.3,186.2 341.5,199.0 377.6,188.4 413.8,182.5 450.0,187.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,205.2 88.2,190.0 124.4,185.3 160.5,183.8 196.7,174.6 232.9,171.7 269.1,159.8 305.3,162.2 341.5,151.1 377.6,131.0 413.8,125.2 450.0,105.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,194.7 88.2,186.0 124.4,188.8 160.5,182.3 196.7,169.1 232.9,166.4 269.1,146.1 305.3,142.9 341.5,127.5 377.6,104.3 413.8,88.4 450.0,78.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.1 88.2,183.0 124.4,172.7 160.5,167.2 196.7,153.1 232.9,146.0 269.1,131.6 305.3,117.8 341.5,103.5 377.6,89.3 413.8,72.2 450.0,59.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.3 88.2,187.3 124.4,166.4 160.5,166.0 196.7,145.5 232.9,139.1 269.1,117.7 305.3,107.6 341.5,93.9 377.6,82.2 413.8,59.0 450.0,49.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.57 µs | 1.87 µs | 2.85 µs | 3.56 µs | 4.2 µs |
| D38 | 1.33 µs | 3.31 µs | 3.91 µs | 4.4 µs | 3.69 µs |
| D57 | 5.01 µs | 4.79 µs | 4.22 µs | 7.99 µs | 8.52 µs |
| D76 | 6.27 µs | 5.12 µs | 5.39 µs | 10.3 µs | 10.1 µs |
| D115 | 13.6 µs | 13.1 µs | 7.78 µs | 22 µs | 28.9 µs |
| D153 | 4.15 µs | 8.32 µs | 11.6 µs | 24.5 µs | 31.4 µs |
| D230 | 5.91 µs | 14.2 µs | 23.2 µs | 44.1 µs | 80.9 µs |
| D307 | 5.96 µs | 12.5 µs | 43.5 µs | 80.8 µs | 119 µs |
| D462 | 5.28 µs | 23.5 µs | 59.2 µs | 160 µs | 225 µs |
| D616 | 4.97 µs | 46.7 µs | 142 µs | 271 µs | 356 µs |
| D924 | 6.99 µs | 59.5 µs | 293 µs | 556 µs | 880 µs |
| D1232 | 5.98 µs | 135 µs | 411 µs | 905 µs | 2.01 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.2 88.2,203.8 124.4,175.0 160.5,170.1 196.7,153.4 232.9,179.1 269.1,171.4 305.3,171.2 341.5,173.9 377.6,175.2 413.8,167.8 450.0,171.2 450.0,44.9 413.8,62.8 377.6,82.4 341.5,92.4 305.3,106.3 269.1,114.6 232.9,135.2 196.7,137.0 160.5,159.8 124.4,163.5 88.2,181.7 52.0,178.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.2 88.2,203.8 124.4,175.0 160.5,170.1 196.7,153.4 232.9,179.1 269.1,171.4 305.3,171.2 341.5,173.9 377.6,175.2 413.8,167.8 450.0,171.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,196.4 88.2,184.0 124.4,176.0 160.5,174.6 196.7,154.1 232.9,164.0 269.1,152.4 305.3,155.2 341.5,141.4 377.6,126.6 413.8,121.3 450.0,103.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.3 88.2,180.4 124.4,178.8 160.5,173.4 196.7,165.4 232.9,156.8 269.1,141.7 305.3,128.1 341.5,121.4 377.6,102.4 413.8,86.6 450.0,79.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.5 88.2,177.8 124.4,164.9 160.5,159.4 196.7,142.9 232.9,140.6 269.1,127.8 305.3,114.6 341.5,99.8 377.6,88.4 413.8,72.8 450.0,62.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.9 88.2,181.7 124.4,163.5 160.5,159.8 196.7,137.0 232.9,135.2 269.1,114.6 305.3,106.3 341.5,92.4 377.6,82.4 413.8,62.8 450.0,44.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.34 µs | 1.13 µs | 1.87 µs | 2.57 µs | 3.06 µs |
| D38 | 1.17 µs | 2.37 µs | 2.87 µs | 3.33 µs | 2.83 µs |
| D57 | 2.78 µs | 2.96 µs | 2.52 µs | 5.46 µs | 7.46 µs |
| D76 | 3.34 µs | 3.18 µs | 3.42 µs | 7.11 µs | 7.36 µs |
| D115 | 3.35 µs | 4.94 µs | 6.6 µs | 13.5 µs | 18.9 µs |
| D153 | 2.05 µs | 5.54 µs | 7.25 µs | 19.5 µs | 25.6 µs |
| D230 | 3.22 µs | 10.3 µs | 19.2 µs | 36.1 µs | 66.9 µs |
| D307 | 3.1 µs | 8.74 µs | 21.1 µs | 66.2 µs | 109 µs |
| D462 | 1.65 µs | 15.4 µs | 43.7 µs | 134 µs | 205 µs |
| D616 | 2.81 µs | 37.8 µs | 126 µs | 256 µs | 358 µs |
| D924 | 3.6 µs | 49.4 µs | 268 µs | 566 µs | 1.01 ms |
| D1232 | 3.04 µs | 120 µs | 423 µs | 1.04 ms | 1.61 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.7 88.2,206.6 124.4,187.8 160.5,183.8 196.7,183.8 232.9,194.4 269.1,184.6 305.3,185.4 341.5,199.1 377.6,187.5 413.8,182.2 450.0,185.8 450.0,49.7 413.8,59.9 377.6,82.3 341.5,94.5 305.3,108.1 269.1,118.7 232.9,139.6 196.7,146.1 160.5,166.7 124.4,166.4 88.2,187.4 52.0,185.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.7 88.2,206.6 124.4,187.8 160.5,183.8 196.7,183.8 232.9,194.4 269.1,184.6 305.3,185.4 341.5,199.1 377.6,187.5 413.8,182.2 450.0,185.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,207.3 88.2,191.3 124.4,186.4 160.5,184.9 196.7,175.3 232.9,172.8 269.1,159.5 305.3,162.9 341.5,150.7 377.6,131.1 413.8,125.3 450.0,106.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,196.4 88.2,187.1 124.4,190.0 160.5,183.3 196.7,169.0 232.9,167.0 269.1,145.8 305.3,143.8 341.5,128.0 377.6,105.0 413.8,88.6 450.0,78.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.5 88.2,183.9 124.4,173.1 160.5,167.4 196.7,153.5 232.9,145.5 269.1,132.1 305.3,118.9 341.5,103.7 377.6,89.6 413.8,72.4 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.7 88.2,187.4 124.4,166.4 160.5,166.7 196.7,146.1 232.9,139.6 269.1,118.7 305.3,108.1 341.5,94.5 377.6,82.3 413.8,59.9 450.0,49.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.57 µs | 1.88 µs | 2.85 µs | 3.56 µs | 4.2 µs |
| D38 | 1.33 µs | 3.32 µs | 3.93 µs | 4.42 µs | 3.74 µs |
| D57 | 5.03 µs | 4.79 µs | 4.23 µs | 8.04 µs | 8.53 µs |
| D76 | 6.3 µs | 5.12 µs | 5.43 µs | 10.3 µs | 10.1 µs |
| D115 | 13.5 µs | 13 µs | 7.9 µs | 22 µs | 28.5 µs |
| D153 | 4.08 µs | 8.36 µs | 11.5 µs | 24.5 µs | 31.3 µs |
| D230 | 5.91 µs | 14.2 µs | 23.3 µs | 44.9 µs | 81.2 µs |
| D307 | 6.02 µs | 12.5 µs | 42.9 µs | 81 µs | 118 µs |
| D462 | 5.32 µs | 23.6 µs | 59.5 µs | 158 µs | 226 µs |
| D616 | 5.01 µs | 47 µs | 143 µs | 272 µs | 354 µs |
| D924 | 7.1 µs | 59.3 µs | 293 µs | 556 µs | 886 µs |
| D1232 | 6.05 µs | 134 µs | 412 µs | 904 µs | 2.01 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.2 88.2,203.8 124.4,174.9 160.5,170.0 196.7,153.4 232.9,179.4 269.1,171.4 305.3,171.0 341.5,173.7 377.6,175.0 413.8,167.5 450.0,170.9 450.0,44.9 413.8,62.6 377.6,82.5 341.5,92.3 305.3,106.3 269.1,114.5 232.9,135.2 196.7,137.3 160.5,159.7 124.4,163.5 88.2,181.3 52.0,178.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.2 88.2,203.8 124.4,174.9 160.5,170.0 196.7,153.4 232.9,179.4 269.1,171.4 305.3,171.0 341.5,173.7 377.6,175.0 413.8,167.5 450.0,170.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,196.3 88.2,183.9 124.4,176.0 160.5,174.5 196.7,154.4 232.9,163.9 269.1,152.4 305.3,155.1 341.5,141.4 377.6,126.4 413.8,121.3 450.0,103.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.2 88.2,180.3 124.4,178.7 160.5,173.3 196.7,165.1 232.9,157.0 269.1,141.6 305.3,128.4 341.5,121.3 377.6,102.3 413.8,86.6 450.0,79.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.4 88.2,177.7 124.4,164.7 160.5,159.4 196.7,142.9 232.9,140.6 269.1,127.4 305.3,114.6 341.5,100.0 377.6,88.3 413.8,72.7 450.0,62.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.8 88.2,181.3 124.4,163.5 160.5,159.7 196.7,137.3 232.9,135.2 269.1,114.5 305.3,106.3 341.5,92.3 377.6,82.5 413.8,62.6 450.0,44.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.48 µs | 2.28 µs | 3.45 µs | 4.24 µs | 5.16 µs |
| D38 | 2.17 µs | 3.98 µs | 4.79 µs | 5.54 µs | 4.98 µs |
| D57 | 3.51 µs | 3.8 µs | 3.32 µs | 7.1 µs | 7.61 µs |
| D76 | 4.29 µs | 4.07 µs | 4.59 µs | 9.09 µs | 9.19 µs |
| D115 | 4.28 µs | 6.43 µs | 7.98 µs | 16.7 µs | 22.8 µs |
| D153 | 2.83 µs | 7.45 µs | 9.13 µs | 23 µs | 30 µs |
| D230 | 4.1 µs | 12.8 µs | 22.8 µs | 41.7 µs | 75.9 µs |
| D307 | 3.94 µs | 10.6 µs | 25.6 µs | 75.9 µs | 121 µs |
| D462 | 2.15 µs | 18.7 µs | 51.2 µs | 148 µs | 221 µs |
| D616 | 3.29 µs | 44.1 µs | 141 µs | 281 µs | 389 µs |
| D924 | 4.58 µs | 56.8 µs | 298 µs | 615 µs | 1.1 ms |
| D1232 | 3.85 µs | 133 µs | 456 µs | 1.09 ms | 1.69 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.3 88.2,193.2 124.4,182.7 160.5,178.4 196.7,178.4 232.9,187.4 269.1,179.4 305.3,180.2 341.5,193.4 377.6,184.1 413.8,177.0 450.0,180.7 450.0,48.6 413.8,57.8 377.6,80.5 341.5,92.7 305.3,105.9 269.1,116.0 232.9,136.2 196.7,142.1 160.5,161.8 124.4,165.9 88.2,175.1 52.0,174.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.3 88.2,193.2 124.4,182.7 160.5,178.4 196.7,178.4 232.9,187.4 269.1,179.4 305.3,180.2 341.5,193.4 377.6,184.1 413.8,177.0 450.0,180.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,192.1 88.2,180.0 124.4,181.0 160.5,179.5 196.7,169.6 232.9,166.4 269.1,154.6 305.3,158.7 341.5,146.5 377.6,127.8 413.8,122.3 450.0,103.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.1 88.2,176.0 124.4,183.9 160.5,176.9 196.7,164.9 232.9,162.0 269.1,142.1 305.3,139.6 341.5,124.5 377.6,102.6 413.8,86.3 450.0,77.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.6 88.2,172.8 124.4,167.4 160.5,162.1 196.7,148.8 232.9,141.9 269.1,129.0 305.3,116.0 341.5,101.5 377.6,87.6 413.8,70.6 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.4 88.2,175.1 124.4,165.9 160.5,161.8 196.7,142.1 232.9,136.2 269.1,116.0 305.3,105.9 341.5,92.7 377.6,80.5 413.8,57.8 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.67 µs | 1.91 µs | 3.01 µs | 3.73 µs | 4.41 µs |
| D38 | 1.43 µs | 3.47 µs | 3.97 µs | 4.42 µs | 3.78 µs |
| D57 | 5.11 µs | 4.9 µs | 4.39 µs | 8.42 µs | 8.92 µs |
| D76 | 6.36 µs | 5.24 µs | 5.64 µs | 10.7 µs | 10.5 µs |
| D115 | 14.1 µs | 13.5 µs | 8.32 µs | 22.9 µs | 29.5 µs |
| D153 | 3.98 µs | 8.75 µs | 12.1 µs | 25.1 µs | 32.6 µs |
| D230 | 6.05 µs | 15.1 µs | 24 µs | 45 µs | 82.6 µs |
| D307 | 6.21 µs | 12.8 µs | 45.1 µs | 82.6 µs | 120 µs |
| D462 | 5.54 µs | 24.3 µs | 60.9 µs | 160 µs | 229 µs |
| D616 | 5.17 µs | 47.6 µs | 145 µs | 275 µs | 356 µs |
| D924 | 7.42 µs | 61 µs | 295 µs | 562 µs | 912 µs |
| D1232 | 6.45 µs | 137 µs | 417 µs | 918 µs | 2.03 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.9 88.2,202.2 124.4,174.6 160.5,169.8 196.7,152.6 232.9,180.0 269.1,170.9 305.3,170.4 341.5,172.8 377.6,174.3 413.8,166.5 450.0,169.5 450.0,44.7 413.8,62.0 377.6,82.5 341.5,92.0 305.3,106.0 269.1,114.1 232.9,134.3 196.7,136.5 160.5,159.0 124.4,162.5 88.2,181.1 52.0,177.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.9 88.2,202.2 124.4,174.6 160.5,169.8 196.7,152.6 232.9,180.0 269.1,170.9 305.3,170.4 341.5,172.8 377.6,174.3 413.8,166.5 450.0,169.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,195.9 88.2,183.0 124.4,175.5 160.5,174.0 196.7,153.4 232.9,162.9 269.1,151.0 305.3,154.6 341.5,140.7 377.6,126.1 413.8,120.7 450.0,103.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.1 88.2,180.1 124.4,177.9 160.5,172.5 196.7,164.0 232.9,155.8 269.1,141.0 305.3,127.3 341.5,120.8 377.6,102.0 413.8,86.5 450.0,79.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.4 88.2,177.7 124.4,163.7 160.5,158.4 196.7,142.0 232.9,140.0 269.1,127.4 305.3,114.2 341.5,99.7 377.6,88.0 413.8,72.5 450.0,61.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,177.8 88.2,181.1 124.4,162.5 160.5,159.0 196.7,136.5 232.9,134.3 269.1,114.1 305.3,106.0 341.5,92.0 377.6,82.5 413.8,62.0 450.0,44.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 134 ns | 148 ns | 141 ns | 162 ns | 190 ns |
| D38 | 99.7 ns | 161 ns | 165 ns | 184 ns | 150 ns |
| D57 | 202 ns | 259 ns | 184 ns | 327 ns | 347 ns |
| D76 | 257 ns | 254 ns | 223 ns | 416 ns | 380 ns |
| D115 | 560 ns | 663 ns | 411 ns | 797 ns | 1.03 µs |
| D153 | 263 ns | 685 ns | 508 ns | 983 ns | 887 ns |
| D230 | 600 ns | 816 ns | 990 ns | 1.3 µs | 1.83 µs |
| D307 | 974 ns | 856 ns | 1.33 µs | 2.17 µs | 2.66 µs |
| D462 | 831 ns | 1.47 µs | 1.58 µs | 3.09 µs | 3.89 µs |
| D616 | 971 ns | 2.38 µs | 3.25 µs | 4.22 µs | 4.8 µs |
| D924 | 1.89 µs | 2.02 µs | 4.94 µs | 7.02 µs | 10.4 µs |
| D1232 | 1.9 µs | 4.3 µs | 7.13 µs | 11.1 µs | 22 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.6 88.2,160.1 124.4,144.7 160.5,139.5 196.7,122.6 232.9,139.0 269.1,121.1 305.3,110.6 341.5,114.0 377.6,110.6 413.8,96.1 450.0,96.0 450.0,42.9 413.8,59.2 377.6,76.0 341.5,80.5 305.3,88.7 269.1,96.9 232.9,112.6 196.7,109.3 160.5,131.0 124.4,133.0 88.2,151.2 52.0,146.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.6 88.2,160.1 124.4,144.7 160.5,139.5 196.7,122.6 232.9,139.0 269.1,121.1 305.3,110.6 341.5,114.0 377.6,110.6 413.8,96.1 450.0,96.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,151.5 88.2,149.7 124.4,139.3 160.5,139.8 196.7,118.9 232.9,118.2 269.1,114.4 305.3,113.4 341.5,101.7 377.6,91.1 413.8,94.8 450.0,78.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.6 88.2,149.1 124.4,146.7 160.5,142.5 196.7,129.3 232.9,124.7 269.1,110.2 305.3,103.8 341.5,100.1 377.6,84.4 413.8,75.3 450.0,67.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,149.6 88.2,146.7 124.4,134.3 160.5,129.1 196.7,114.9 232.9,110.4 269.1,104.4 305.3,93.2 341.5,85.5 377.6,78.8 413.8,67.7 450.0,57.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,146.0 88.2,151.2 124.4,133.0 160.5,131.0 196.7,109.3 232.9,112.6 269.1,96.9 305.3,88.7 341.5,80.5 377.6,76.0 413.8,59.2 450.0,42.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 155 ns | 168 ns | 186 ns | 202 ns | 224 ns |
| D38 | 136 ns | 194 ns | 197 ns | 198 ns | 187 ns |
| D57 | 269 ns | 312 ns | 211 ns | 409 ns | 420 ns |
| D76 | 340 ns | 313 ns | 261 ns | 496 ns | 456 ns |
| D115 | 728 ns | 786 ns | 478 ns | 945 ns | 1.06 µs |
| D153 | 377 ns | 788 ns | 564 ns | 1.08 µs | 1.03 µs |
| D230 | 829 ns | 995 ns | 1.14 µs | 1.51 µs | 2.02 µs |
| D307 | 1.3 µs | 1.04 µs | 1.6 µs | 2.47 µs | 3.02 µs |
| D462 | 1.1 µs | 1.65 µs | 1.88 µs | 3.46 µs | 4.29 µs |
| D616 | 1.22 µs | 2.62 µs | 3.52 µs | 4.66 µs | 5.24 µs |
| D924 | 2.42 µs | 2.32 µs | 5.51 µs | 7.6 µs | 10.9 µs |
| D1232 | 2.5 µs | 4.88 µs | 7.81 µs | 12 µs | 25.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.2 88.2,201.1 124.4,181.4 160.5,174.6 196.7,152.5 232.9,171.6 269.1,148.8 305.3,135.7 341.5,140.7 377.6,137.5 413.8,117.7 450.0,116.9 450.0,49.8 413.8,74.1 377.6,95.4 341.5,101.2 305.3,111.3 269.1,123.0 232.9,142.6 196.7,141.6 160.5,166.1 124.4,168.5 88.2,191.9 52.0,186.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.2 88.2,201.1 124.4,181.4 160.5,174.6 196.7,152.5 232.9,171.6 269.1,148.8 305.3,135.7 341.5,140.7 377.6,137.5 413.8,117.7 450.0,116.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,195.0 88.2,190.9 124.4,177.0 160.5,176.9 196.7,150.3 232.9,150.2 269.1,143.5 305.3,142.2 341.5,128.8 377.6,115.5 413.8,119.0 450.0,97.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,192.0 88.2,190.3 124.4,188.4 160.5,182.2 196.7,164.7 232.9,159.9 269.1,139.7 305.3,129.8 341.5,125.1 377.6,106.9 413.8,93.9 450.0,83.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.7 88.2,190.2 124.4,169.2 160.5,163.6 196.7,145.0 232.9,141.2 269.1,131.4 305.3,117.2 341.5,107.4 377.6,98.8 413.8,84.6 450.0,71.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.6 88.2,191.9 124.4,168.5 160.5,166.1 196.7,141.6 232.9,142.6 269.1,123.0 305.3,111.3 341.5,101.2 377.6,95.4 413.8,74.1 450.0,49.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
