# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.32 ns | 1.59 µs | 3.21 µs | 2.75 µs | 3.81 µs |
| D38 | 4.13 µs | 5.55 µs | 7.46 µs | 11.3 µs | 14.7 µs |
| D57 | 4.97 µs | 7.39 µs | 11.7 µs | 15.5 µs | 18.2 µs |
| D76 | 4.7 µs | 9.1 µs | 15.3 µs | 20.3 µs | 21 µs |
| D115 | 3.6 µs | 9.3 µs | 26.1 µs | 32.4 µs | 41.7 µs |
| D153 | 3.66 µs | 9 µs | 28.6 µs | 42.5 µs | 65.7 µs |
| D230 | 5.21 µs | 25 µs | 41.8 µs | 76.4 µs | 122 µs |
| D307 | 3.75 µs | 28.5 µs | 60.5 µs | 124 µs | 191 µs |
| D462 | 5.43 µs | 45.4 µs | 109 µs | 241 µs | 377 µs |
| D616 | 4.35 µs | 61.2 µs | 206 µs | 424 µs | 657 µs |
| D924 | 5.4 µs | 135 µs | 451 µs | 774 µs | 1.68 ms |
| D1232 | 5.65 µs | 220 µs | 712 µs | 1.16 ms | 3.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.3 88.2,106.7 124.4,104.4 160.5,105.1 196.7,108.4 232.9,108.2 269.1,103.8 305.3,107.9 341.5,103.3 377.6,106.0 413.8,103.4 450.0,102.8 450.0,23.0 413.8,32.2 377.6,43.8 341.5,50.7 305.3,59.1 269.1,64.7 232.9,72.4 196.7,78.0 160.5,86.5 124.4,88.3 88.2,90.9 52.0,107.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.3 88.2,106.7 124.4,104.4 160.5,105.1 196.7,108.4 232.9,108.2 269.1,103.8 305.3,107.9 341.5,103.3 377.6,106.0 413.8,103.4 450.0,102.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.5 88.2,103.0 124.4,99.5 160.5,96.9 196.7,96.6 232.9,97.0 269.1,84.4 305.3,82.7 341.5,76.9 377.6,73.2 413.8,63.4 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,99.3 124.4,93.7 160.5,90.4 196.7,83.8 232.9,82.7 269.1,78.0 305.3,73.4 341.5,66.1 377.6,58.2 413.8,48.4 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.7 88.2,94.2 124.4,90.2 160.5,86.9 196.7,81.1 232.9,77.8 269.1,70.5 305.3,64.5 341.5,56.2 377.6,49.2 413.8,41.7 450.0,36.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,90.9 124.4,88.3 160.5,86.5 196.7,78.0 232.9,72.4 269.1,64.7 305.3,59.1 341.5,50.7 377.6,43.8 413.8,32.2 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.58 µs | 2.88 µs | 5.85 µs | 4.99 µs | 6.79 µs |
| D38 | 3.28 µs | 4.62 µs | 7 µs | 8.22 µs | 9.48 µs |
| D57 | 3.36 µs | 3.41 µs | 5.04 µs | 6.18 µs | 7.63 µs |
| D76 | 3.34 µs | 4.53 µs | 5.78 µs | 8.42 µs | 9.29 µs |
| D115 | 4.93 µs | 7.36 µs | 13.7 µs | 17.4 µs | 22.1 µs |
| D153 | 4.9 µs | 6.59 µs | 15.5 µs | 22.2 µs | 34.6 µs |
| D230 | 9.28 µs | 16.5 µs | 27.5 µs | 48 µs | 70.9 µs |
| D307 | 9.41 µs | 26.6 µs | 46.4 µs | 83.2 µs | 137 µs |
| D462 | 13.1 µs | 37.6 µs | 70.6 µs | 159 µs | 259 µs |
| D616 | 18.4 µs | 71.6 µs | 171 µs | 314 µs | 522 µs |
| D924 | 34.7 µs | 164 µs | 406 µs | 671 µs | 1.45 ms |
| D1232 | 45.2 µs | 275 µs | 724 µs | 1.14 ms | 3.09 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.4 88.2,184.2 124.4,183.7 160.5,183.8 196.7,175.4 232.9,175.5 269.1,161.6 305.3,161.3 341.5,154.1 377.6,146.7 413.8,133.0 450.0,127.2 450.0,35.5 413.8,52.0 377.6,74.1 341.5,89.4 305.3,103.2 269.1,117.5 232.9,133.1 196.7,142.8 160.5,161.6 124.4,165.9 88.2,161.2 52.0,168.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.4 88.2,184.2 124.4,183.7 160.5,183.8 196.7,175.4 232.9,175.5 269.1,161.6 305.3,161.3 341.5,154.1 377.6,146.7 413.8,133.0 450.0,127.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,187.0 88.2,176.8 124.4,183.4 160.5,177.2 196.7,166.6 232.9,169.0 269.1,149.2 305.3,138.8 341.5,131.2 377.6,117.3 413.8,99.3 450.0,88.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.7 88.2,167.8 124.4,174.9 160.5,171.9 196.7,153.2 232.9,150.5 269.1,138.0 305.3,126.7 341.5,117.6 377.6,98.3 413.8,79.6 450.0,67.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,175.1 88.2,164.2 124.4,170.5 160.5,163.7 196.7,148.0 232.9,142.7 269.1,125.9 305.3,114.0 341.5,100.0 377.6,85.1 413.8,68.7 450.0,57.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.4 88.2,161.2 124.4,165.9 160.5,161.6 196.7,142.8 232.9,133.1 269.1,117.5 305.3,103.2 341.5,89.4 377.6,74.1 413.8,52.0 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.39 ns | 1.6 µs | 3.19 µs | 2.72 µs | 3.79 µs |
| D38 | 4.03 µs | 5.54 µs | 7.41 µs | 11.3 µs | 14.7 µs |
| D57 | 4.59 µs | 7.46 µs | 11.7 µs | 15.5 µs | 18.2 µs |
| D76 | 4.61 µs | 9 µs | 15.2 µs | 20.2 µs | 21 µs |
| D115 | 3.49 µs | 9.26 µs | 26.3 µs | 32.7 µs | 41.5 µs |
| D153 | 3.53 µs | 8.95 µs | 28.4 µs | 42.1 µs | 66.6 µs |
| D230 | 5.12 µs | 24.7 µs | 42.2 µs | 75.6 µs | 122 µs |
| D307 | 3.65 µs | 28.2 µs | 60.5 µs | 124 µs | 191 µs |
| D462 | 5.13 µs | 45.1 µs | 108 µs | 240 µs | 376 µs |
| D616 | 4.18 µs | 61.2 µs | 205 µs | 424 µs | 657 µs |
| D924 | 5.28 µs | 134 µs | 451 µs | 774 µs | 1.68 ms |
| D1232 | 5.65 µs | 219 µs | 713 µs | 1.17 ms | 3.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.8 88.2,107.0 124.4,105.4 160.5,105.3 196.7,108.8 232.9,108.6 269.1,104.0 305.3,108.2 341.5,104.0 377.6,106.5 413.8,103.6 450.0,102.8 450.0,23.0 413.8,32.1 377.6,43.8 341.5,50.7 305.3,59.1 269.1,64.7 232.9,72.2 196.7,78.1 160.5,86.5 124.4,88.3 88.2,90.9 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.8 88.2,107.0 124.4,105.4 160.5,105.3 196.7,108.8 232.9,108.6 269.1,104.0 305.3,108.2 341.5,104.0 377.6,106.5 413.8,103.6 450.0,102.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.5 88.2,103.0 124.4,99.3 160.5,97.0 196.7,96.7 232.9,97.1 269.1,84.5 305.3,82.9 341.5,77.0 377.6,73.2 413.8,63.5 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,99.4 124.4,93.7 160.5,90.5 196.7,83.7 232.9,82.8 269.1,77.9 305.3,73.4 341.5,66.2 377.6,58.2 413.8,48.4 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.9 88.2,94.2 124.4,90.2 160.5,87.0 196.7,81.0 232.9,77.9 269.1,70.6 305.3,64.5 341.5,56.3 377.6,49.2 413.8,41.7 450.0,36.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,90.9 124.4,88.3 160.5,86.5 196.7,78.1 232.9,72.2 269.1,64.7 305.3,59.1 341.5,50.7 377.6,43.8 413.8,32.1 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.52 ns | 1.39 µs | 2.54 µs | 2.23 µs | 2.96 µs |
| D38 | 3.73 ns | 2.01 µs | 3.08 µs | 3.67 µs | 4.19 µs |
| D57 | 1.95 ns | 4.33 µs | 7.43 µs | 9.2 µs | 11.8 µs |
| D76 | 2.02 ns | 6.23 µs | 8.8 µs | 13.2 µs | 14.1 µs |
| D115 | 9.2 ns | 10.9 µs | 20.2 µs | 24.5 µs | 32.6 µs |
| D153 | 11.5 ns | 9.91 µs | 22.1 µs | 33.7 µs | 47.8 µs |
| D230 | 31.6 ns | 24.2 µs | 41.4 µs | 66.5 µs | 95.3 µs |
| D307 | 39.3 ns | 38 µs | 68.6 µs | 115 µs | 180 µs |
| D462 | 75.4 ns | 59.5 µs | 99.5 µs | 198 µs | 319 µs |
| D616 | 83.4 ns | 106 µs | 224 µs | 400 µs | 613 µs |
| D924 | 112 ns | 252 µs | 532 µs | 752 µs | 1.6 ms |
| D1232 | 158 ns | 389 µs | 924 µs | 1.23 ms | 3.11 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,193.7 124.4,201.7 160.5,201.3 196.7,182.5 232.9,179.7 269.1,167.1 305.3,164.5 341.5,156.4 377.6,155.1 413.8,151.4 450.0,147.2 450.0,24.5 413.8,32.7 377.6,44.6 341.5,52.7 305.3,59.8 269.1,67.7 232.9,76.3 196.7,81.1 160.5,91.4 124.4,93.7 88.2,106.5 52.0,110.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,193.7 124.4,201.7 160.5,201.3 196.7,182.5 232.9,179.7 269.1,167.1 305.3,164.5 341.5,156.4 377.6,155.1 413.8,151.4 450.0,147.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.2 88.2,115.6 124.4,106.1 160.5,101.6 196.7,94.7 232.9,95.8 269.1,84.8 305.3,79.1 341.5,73.6 377.6,66.5 413.8,55.7 450.0,50.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,110.3 124.4,99.4 160.5,97.3 196.7,87.0 232.9,85.9 269.1,78.1 305.3,71.8 341.5,67.2 377.6,57.1 413.8,46.4 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.3 88.2,108.2 124.4,96.7 160.5,92.3 196.7,84.6 232.9,80.7 269.1,72.2 305.3,65.4 341.5,58.7 377.6,49.9 413.8,42.1 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,106.5 124.4,93.7 160.5,91.4 196.7,81.1 232.9,76.3 269.1,67.7 305.3,59.8 341.5,52.7 377.6,44.6 413.8,32.7 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.37 ns | 1.28 µs | 2.69 µs | 2.27 µs | 3.26 µs |
| D38 | 4.71 µs | 5.65 µs | 6.14 µs | 9.82 µs | 12.8 µs |
| D57 | 3.82 µs | 6.34 µs | 10.1 µs | 13.7 µs | 5.28 µs |
| D76 | 3.79 µs | 7.87 µs | 13.3 µs | 17.8 µs | 18.7 µs |
| D115 | 2.9 µs | 8.04 µs | 25 µs | 29.2 µs | 37.6 µs |
| D153 | 2.95 µs | 7.86 µs | 22.2 µs | 38 µs | 60.2 µs |
| D230 | 4.24 µs | 21.9 µs | 38.8 µs | 69.7 µs | 114 µs |
| D307 | 2.97 µs | 25 µs | 49.9 µs | 115 µs | 178 µs |
| D462 | 3.06 µs | 36.9 µs | 90.6 µs | 211 µs | 320 µs |
| D616 | 3.49 µs | 55.8 µs | 190 µs | 395 µs | 621 µs |
| D924 | 4.35 µs | 124 µs | 424 µs | 735 µs | 1.58 ms |
| D1232 | 4.6 µs | 204 µs | 674 µs | 1.13 ms | 3.36 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.9 88.2,105.1 124.4,107.7 160.5,107.7 196.7,111.1 232.9,110.8 269.1,106.4 305.3,110.8 341.5,110.4 377.6,108.8 413.8,106.0 450.0,105.4 450.0,23.5 413.8,32.9 377.6,44.5 341.5,52.7 305.3,60.0 269.1,65.6 232.9,73.4 196.7,79.3 160.5,87.9 124.4,103.6 88.2,92.6 52.0,109.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.9 88.2,105.1 124.4,107.7 160.5,107.7 196.7,111.1 232.9,110.8 269.1,106.4 305.3,110.8 341.5,110.4 377.6,108.8 413.8,106.0 450.0,105.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.2 88.2,102.8 124.4,101.4 160.5,98.7 196.7,98.4 232.9,98.7 269.1,86.0 305.3,84.3 341.5,79.5 377.6,74.4 413.8,64.4 450.0,58.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,101.8 124.4,95.6 160.5,92.2 196.7,84.4 232.9,85.8 269.1,78.9 305.3,75.8 341.5,68.4 377.6,59.2 413.8,49.2 450.0,43.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.1 88.2,95.9 124.4,91.8 160.5,88.6 196.7,82.4 232.9,79.1 269.1,71.6 305.3,65.4 341.5,57.9 377.6,50.1 413.8,42.4 450.0,37.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.6 88.2,92.6 124.4,103.6 160.5,87.9 196.7,79.3 232.9,73.4 269.1,65.6 305.3,60.0 341.5,52.7 377.6,44.5 413.8,32.9 450.0,23.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.57 ns | 1.41 µs | 2.9 µs | 2.47 µs | 3.44 µs |
| D38 | 3.74 ns | 2.29 µs | 3.56 µs | 4.12 µs | 4.79 µs |
| D57 | 437 ns | 4.43 µs | 7.11 µs | 8.96 µs | 11.5 µs |
| D76 | 455 ns | 5.96 µs | 8.08 µs | 12.1 µs | 14 µs |
| D115 | 786 ns | 10.3 µs | 21.1 µs | 26 µs | 34.7 µs |
| D153 | 781 ns | 10.4 µs | 24.2 µs | 35 µs | 57.6 µs |
| D230 | 1.37 µs | 24.5 µs | 42.8 µs | 79.4 µs | 121 µs |
| D307 | 1.38 µs | 40 µs | 76.4 µs | 141 µs | 242 µs |
| D462 | 2.17 µs | 57.2 µs | 121 µs | 283 µs | 470 µs |
| D616 | 2.74 µs | 116 µs | 299 µs | 564 µs | 952 µs |
| D924 | 5.24 µs | 274 µs | 720 µs | 1.24 ms | 2.7 ms |
| D1232 | 7.07 µs | 470 µs | 1.31 ms | 2.09 ms | 5.82 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.3 88.2,193.6 124.4,134.6 160.5,134.1 196.7,127.3 232.9,127.4 269.1,120.3 305.3,120.2 341.5,114.7 377.6,111.8 413.8,103.7 450.0,100.0 450.0,16.7 413.8,26.2 377.6,39.2 341.5,47.9 305.3,56.2 269.1,64.8 232.9,74.0 196.7,80.3 160.5,91.5 124.4,94.0 88.2,104.8 52.0,108.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.3 88.2,193.6 124.4,134.6 160.5,134.1 196.7,127.3 232.9,127.4 269.1,120.3 305.3,120.2 341.5,114.7 377.6,111.8 413.8,103.7 450.0,100.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.0 88.2,114.0 124.4,105.8 160.5,102.1 196.7,95.3 232.9,95.3 269.1,84.6 305.3,78.5 341.5,74.1 377.6,65.3 413.8,54.6 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,108.5 124.4,99.9 160.5,98.4 196.7,86.5 232.9,84.8 269.1,77.7 305.3,70.5 341.5,64.8 377.6,53.6 413.8,42.7 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.1 88.2,106.7 124.4,97.1 160.5,93.3 196.7,83.9 232.9,80.2 269.1,70.0 305.3,62.8 341.5,54.2 377.6,45.7 413.8,35.9 450.0,29.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,104.8 124.4,94.0 160.5,91.5 196.7,80.3 232.9,74.0 269.1,64.8 305.3,56.2 341.5,47.9 377.6,39.2 413.8,26.2 450.0,16.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.53 ns | 1.01 µs | 2.5 µs | 2.21 µs | 2.84 µs |
| D38 | 4.98 ns | 1.75 µs | 2.98 µs | 3.46 µs | 3.78 µs |
| D57 | 2.18 ns | 2.91 µs | 4.72 µs | 5.57 µs | 9.19 µs |
| D76 | 3.43 ns | 3.79 µs | 5.45 µs | 7.82 µs | 9.2 µs |
| D115 | 10.5 ns | 3.56 µs | 10 µs | 13.5 µs | 18.3 µs |
| D153 | 15 ns | 3.47 µs | 9.77 µs | 18.8 µs | 32.7 µs |
| D230 | 40.7 ns | 9.79 µs | 18.2 µs | 39.7 µs | 69.7 µs |
| D307 | 51.6 ns | 12.5 µs | 25 µs | 69.5 µs | 117 µs |
| D462 | 155 ns | 16 µs | 54.2 µs | 135 µs | 218 µs |
| D616 | 131 ns | 30.4 µs | 124 µs | 256 µs | 429 µs |
| D924 | 164 ns | 75.4 µs | 274 µs | 507 µs | 1.13 ms |
| D1232 | 363 ns | 130 µs | 458 µs | 826 µs | 2.46 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.3 88.2,190.1 124.4,200.3 160.5,194.7 196.7,180.9 232.9,176.4 269.1,164.0 305.3,161.1 341.5,147.4 377.6,149.5 413.8,146.7 450.0,136.9 450.0,27.4 413.8,37.0 377.6,49.1 341.5,57.5 305.3,65.2 269.1,71.6 232.9,81.0 196.7,88.2 160.5,96.7 124.4,96.8 88.2,107.8 52.0,111.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.3 88.2,190.1 124.4,200.3 160.5,194.7 196.7,180.9 232.9,176.4 269.1,164.0 305.3,161.1 341.5,147.4 377.6,149.5 413.8,146.7 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.1 88.2,117.3 124.4,111.0 160.5,107.7 196.7,108.5 232.9,108.8 269.1,96.0 305.3,92.9 341.5,89.9 377.6,81.9 413.8,70.6 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.9 88.2,110.8 124.4,105.0 160.5,103.3 196.7,95.7 232.9,96.0 269.1,88.3 305.3,84.3 341.5,74.7 377.6,64.5 413.8,54.7 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.5 88.2,108.9 124.4,103.0 160.5,98.8 196.7,92.0 232.9,87.9 269.1,78.6 305.3,71.7 341.5,63.4 377.6,55.5 413.8,47.0 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,107.8 124.4,96.8 160.5,96.7 196.7,88.2 232.9,81.0 269.1,71.6 305.3,65.2 341.5,57.5 377.6,49.1 413.8,37.0 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.92 ns | 1.5 µs | 3.38 µs | 3.06 µs | 3.84 µs |
| D38 | 4.04 ns | 2.45 µs | 3.98 µs | 4.47 µs | 4.92 µs |
| D57 | 3.11 ns | 4.54 µs | 7.21 µs | 8.18 µs | 10.5 µs |
| D76 | 3.74 ns | 5.94 µs | 7.82 µs | 11.2 µs | 12.2 µs |
| D115 | 6.34 ns | 10.2 µs | 13.2 µs | 21.7 µs | 26 µs |
| D153 | 13.9 ns | 4.69 µs | 15.5 µs | 23.3 µs | 39.6 µs |
| D230 | 40.4 ns | 13.9 µs | 23.2 µs | 47.3 µs | 80.3 µs |
| D307 | 53.2 ns | 17 µs | 51.5 µs | 80.5 µs | 125 µs |
| D462 | 229 ns | 25 µs | 69.2 µs | 157 µs | 234 µs |
| D616 | 130 ns | 36.1 µs | 134 µs | 271 µs | 415 µs |
| D924 | 208 ns | 87.8 µs | 292 µs | 507 µs | 997 µs |
| D1232 | 363 ns | 144 µs | 449 µs | 729 µs | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,196.7 88.2,192.7 124.4,195.9 160.5,193.6 196.7,187.1 232.9,177.4 269.1,164.1 305.3,160.7 341.5,142.6 377.6,149.6 413.8,143.8 450.0,136.9 450.0,25.5 413.8,38.6 377.6,49.5 341.5,56.6 305.3,64.4 269.1,69.9 232.9,78.6 196.7,83.9 160.5,93.3 124.4,95.1 88.2,104.5 52.0,107.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,196.7 88.2,192.7 124.4,195.9 160.5,193.6 196.7,187.1 232.9,177.4 269.1,164.1 305.3,160.7 341.5,142.6 377.6,149.6 413.8,143.8 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.3 88.2,113.2 124.4,105.5 160.5,102.2 196.7,95.5 232.9,105.1 269.1,91.6 305.3,89.1 341.5,84.4 377.6,79.8 413.8,68.8 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.2 88.2,107.2 124.4,99.8 160.5,98.8 196.7,92.3 232.9,90.3 269.1,85.3 305.3,75.4 341.5,71.7 377.6,63.5 413.8,53.9 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,105.7 124.4,98.2 160.5,94.3 196.7,86.1 232.9,85.2 269.1,76.4 305.3,69.8 341.5,61.6 377.6,54.8 413.8,47.0 450.0,42.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,104.5 124.4,95.1 160.5,93.3 196.7,83.9 232.9,78.6 269.1,69.9 305.3,64.4 341.5,56.6 377.6,49.5 413.8,38.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.29 ns | 931 ns | 2.34 µs | 2.17 µs | 2.78 µs |
| D38 | 4.36 ns | 1.6 µs | 2.84 µs | 3.28 µs | 3.75 µs |
| D57 | 2.19 ns | 2.71 µs | 4.41 µs | 5.56 µs | 9.19 µs |
| D76 | 3.43 ns | 3.48 µs | 5.17 µs | 7.73 µs | 8.87 µs |
| D115 | 10.6 ns | 3.38 µs | 10.1 µs | 13.4 µs | 18.1 µs |
| D153 | 13.9 ns | 2.96 µs | 10 µs | 18 µs | 31.8 µs |
| D230 | 40.8 ns | 9.83 µs | 18.3 µs | 39.6 µs | 67 µs |
| D307 | 45.6 ns | 12.1 µs | 24.5 µs | 66 µs | 116 µs |
| D462 | 128 ns | 16.1 µs | 51.2 µs | 131 µs | 216 µs |
| D616 | 122 ns | 29.9 µs | 121 µs | 253 µs | 427 µs |
| D924 | 194 ns | 72.6 µs | 271 µs | 505 µs | 1.13 ms |
| D1232 | 365 ns | 129 µs | 453 µs | 822 µs | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.2 88.2,191.7 124.4,200.3 160.5,194.7 196.7,180.7 232.9,177.3 269.1,164.0 305.3,162.6 341.5,149.8 377.6,150.4 413.8,144.6 450.0,136.8 450.0,27.5 413.8,37.0 377.6,49.1 341.5,57.6 305.3,65.3 269.1,72.1 232.9,81.4 196.7,88.3 160.5,97.2 124.4,96.8 88.2,107.9 52.0,111.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.2 88.2,191.7 124.4,200.3 160.5,194.7 196.7,180.7 232.9,177.3 269.1,164.0 305.3,162.6 341.5,149.8 377.6,150.4 413.8,144.6 450.0,136.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,125.2 88.2,118.4 124.4,111.9 160.5,108.8 196.7,109.2 232.9,110.8 269.1,95.9 305.3,93.3 341.5,89.8 377.6,82.1 413.8,71.1 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.7 88.2,111.3 124.4,105.9 160.5,103.9 196.7,95.6 232.9,95.7 269.1,88.2 305.3,84.6 341.5,75.5 377.6,64.8 413.8,54.8 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.7 88.2,109.5 124.4,103.0 160.5,98.9 196.7,92.1 232.9,88.4 269.1,78.6 305.3,72.3 341.5,63.8 377.6,55.6 413.8,47.1 450.0,41.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.6 88.2,107.9 124.4,96.8 160.5,97.2 196.7,88.3 232.9,81.4 269.1,72.1 305.3,65.3 341.5,57.6 377.6,49.1 413.8,37.0 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.39 ns | 1.5 µs | 3.37 µs | 3.06 µs | 3.86 µs |
| D38 | 4.04 ns | 2.46 µs | 3.97 µs | 4.48 µs | 4.93 µs |
| D57 | 10.4 ns | 4.57 µs | 7.26 µs | 8.2 µs | 10.5 µs |
| D76 | 10.4 ns | 5.97 µs | 7.84 µs | 11.2 µs | 12.2 µs |
| D115 | 6.65 ns | 10.1 µs | 12.7 µs | 21.7 µs | 25.9 µs |
| D153 | 13.2 ns | 4.69 µs | 15.3 µs | 23.3 µs | 39.5 µs |
| D230 | 39.9 ns | 13.9 µs | 23.2 µs | 47.4 µs | 80.9 µs |
| D307 | 46 ns | 16.9 µs | 50.2 µs | 80.9 µs | 125 µs |
| D462 | 124 ns | 25 µs | 70.4 µs | 157 µs | 234 µs |
| D616 | 119 ns | 36.2 µs | 134 µs | 271 µs | 415 µs |
| D924 | 228 ns | 89.4 µs | 293 µs | 504 µs | 998 µs |
| D1232 | 358 ns | 144 µs | 448 µs | 729 µs | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.2 88.2,192.7 124.4,181.0 160.5,180.9 196.7,186.5 232.9,178.0 269.1,164.2 305.3,162.5 341.5,150.2 377.6,150.7 413.8,142.6 450.0,137.0 450.0,25.5 413.8,38.6 377.6,49.5 341.5,56.6 305.3,64.4 269.1,69.8 232.9,78.7 196.7,83.9 160.5,93.2 124.4,95.1 88.2,104.5 52.0,107.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.2 88.2,192.7 124.4,181.0 160.5,180.9 196.7,186.5 232.9,178.0 269.1,164.2 305.3,162.5 341.5,150.2 377.6,150.7 413.8,142.6 450.0,137.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.3 88.2,113.1 124.4,105.4 160.5,102.1 196.7,95.6 232.9,105.1 269.1,91.6 305.3,89.2 341.5,84.3 377.6,79.8 413.8,68.5 450.0,62.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.2 88.2,107.2 124.4,99.7 160.5,98.7 196.7,92.7 232.9,90.4 269.1,85.2 305.3,75.7 341.5,71.5 377.6,63.5 413.8,53.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,105.7 124.4,98.2 160.5,94.3 196.7,86.1 232.9,85.2 269.1,76.4 305.3,69.8 341.5,61.5 377.6,54.8 413.8,47.1 450.0,42.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.5 88.2,104.5 124.4,95.1 160.5,93.2 196.7,83.9 232.9,78.7 269.1,69.8 305.3,64.4 341.5,56.6 377.6,49.5 413.8,38.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.3 ns | 1.82 µs | 3.96 µs | 3.57 µs | 4.58 µs |
| D38 | 4.36 ns | 3 µs | 4.73 µs | 5.48 µs | 6.08 µs |
| D57 | 3.11 ns | 3.59 µs | 5.77 µs | 7.15 µs | 9.07 µs |
| D76 | 4.05 ns | 4.59 µs | 6.89 µs | 9.88 µs | 11 µs |
| D115 | 7.08 ns | 4.53 µs | 12.8 µs | 16.3 µs | 21.2 µs |
| D153 | 15 ns | 3.94 µs | 12.3 µs | 22 µs | 37.2 µs |
| D230 | 41.2 ns | 12.4 µs | 22.1 µs | 45.1 µs | 75.7 µs |
| D307 | 44.1 ns | 14.9 µs | 29.1 µs | 75.3 µs | 126 µs |
| D462 | 114 ns | 19.7 µs | 58.1 µs | 147 µs | 233 µs |
| D616 | 175 ns | 34.4 µs | 133 µs | 280 µs | 464 µs |
| D924 | 159 ns | 82.4 µs | 296 µs | 542 µs | 1.2 ms |
| D1232 | 351 ns | 140 µs | 492 µs | 869 µs | 2.59 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.2 88.2,191.7 124.4,195.9 160.5,192.6 196.7,185.7 232.9,176.4 269.1,163.9 305.3,163.0 341.5,151.2 377.6,145.9 413.8,147.1 450.0,137.3 450.0,26.8 413.8,36.3 377.6,48.1 341.5,56.6 305.3,64.3 269.1,70.6 232.9,79.4 196.7,86.4 160.5,94.6 124.4,96.9 88.2,101.9 52.0,105.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.2 88.2,191.7 124.4,195.9 160.5,192.6 196.7,185.7 232.9,176.4 269.1,163.9 305.3,163.0 341.5,151.2 377.6,145.9 413.8,147.1 450.0,137.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.8 88.2,110.7 124.4,108.4 160.5,105.4 196.7,105.5 232.9,107.3 269.1,93.1 305.3,90.8 341.5,87.3 377.6,80.4 413.8,69.5 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.2 88.2,105.0 124.4,102.5 160.5,100.3 196.7,92.7 232.9,93.2 269.1,85.9 305.3,82.5 341.5,73.9 377.6,63.6 413.8,53.7 450.0,47.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,103.2 124.4,99.9 160.5,95.9 196.7,89.6 232.9,85.9 269.1,77.0 305.3,70.7 341.5,62.3 377.6,54.4 413.8,46.2 450.0,40.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.4 88.2,101.9 124.4,96.9 160.5,94.6 196.7,86.4 232.9,79.4 269.1,70.6 305.3,64.3 341.5,56.6 377.6,48.1 413.8,36.3 450.0,26.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.25 ns | 1.51 µs | 3.55 µs | 3.2 µs | 4.02 µs |
| D38 | 3.73 ns | 2.6 µs | 4.01 µs | 4.49 µs | 4.95 µs |
| D57 | 2.59 µs | 4.69 µs | 7.64 µs | 8.62 µs | 10.9 µs |
| D76 | 2.56 µs | 6.15 µs | 8.17 µs | 11.7 µs | 12.7 µs |
| D115 | 4.29 µs | 10.6 µs | 13.6 µs | 22.2 µs | 26.8 µs |
| D153 | 2.18 µs | 4.88 µs | 16.1 µs | 24.1 µs | 40.9 µs |
| D230 | 3.07 µs | 14.7 µs | 23.8 µs | 48.4 µs | 82.3 µs |
| D307 | 2.37 µs | 17.6 µs | 52.1 µs | 82.4 µs | 126 µs |
| D462 | 3.36 µs | 25.9 µs | 70.8 µs | 160 µs | 237 µs |
| D616 | 2.88 µs | 37.1 µs | 137 µs | 274 µs | 419 µs |
| D924 | 3.79 µs | 90.9 µs | 296 µs | 513 µs | 1.01 ms |
| D1232 | 4.47 µs | 147 µs | 455 µs | 736 µs | 2.89 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.9 88.2,193.6 124.4,112.5 160.5,112.6 196.7,106.2 232.9,114.6 269.1,110.3 305.3,113.6 341.5,109.3 377.6,111.2 413.8,107.7 450.0,105.7 450.0,25.4 413.8,38.5 377.6,49.4 341.5,56.4 305.3,64.2 269.1,69.6 232.9,78.2 196.7,83.5 160.5,92.8 124.4,94.7 88.2,104.5 52.0,107.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.9 88.2,193.6 124.4,112.5 160.5,112.6 196.7,106.2 232.9,114.6 269.1,110.3 305.3,113.6 341.5,109.3 377.6,111.2 413.8,107.7 450.0,105.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.2 88.2,112.4 124.4,105.1 160.5,101.8 196.7,95.0 232.9,104.6 269.1,91.0 305.3,88.7 341.5,83.9 377.6,79.5 413.8,68.3 450.0,62.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.6 88.2,107.1 124.4,99.1 160.5,98.2 196.7,91.9 232.9,89.8 269.1,84.9 305.3,75.2 341.5,71.4 377.6,63.3 413.8,53.7 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,105.7 124.4,97.6 160.5,93.8 196.7,85.8 232.9,84.8 269.1,76.1 305.3,69.5 341.5,61.3 377.6,54.6 413.8,46.9 450.0,42.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,104.5 124.4,94.7 160.5,92.8 196.7,83.5 232.9,78.2 269.1,69.6 305.3,64.2 341.5,56.4 377.6,49.4 413.8,38.5 450.0,25.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.96 ns | 92.6 ns | 162 ns | 134 ns | 164 ns |
| D38 | 4.36 ns | 116 ns | 167 ns | 181 ns | 189 ns |
| D57 | 182 ns | 228 ns | 320 ns | 326 ns | 415 ns |
| D76 | 179 ns | 298 ns | 327 ns | 445 ns | 436 ns |
| D115 | 268 ns | 412 ns | 803 ns | 821 ns | 885 ns |
| D153 | 293 ns | 370 ns | 746 ns | 886 ns | 1.16 µs |
| D230 | 565 ns | 838 ns | 994 ns | 1.44 µs | 1.82 µs |
| D307 | 561 ns | 1.17 µs | 1.5 µs | 2.07 µs | 2.78 µs |
| D462 | 934 ns | 1.54 µs | 1.83 µs | 3.01 µs | 3.89 µs |
| D616 | 878 ns | 1.84 µs | 2.88 µs | 4.12 µs | 5.67 µs |
| D924 | 1.58 µs | 2.94 µs | 4.86 µs | 6.23 µs | 11.3 µs |
| D1232 | 2.27 µs | 4.24 µs | 7.4 µs | 8.69 µs | 30.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,184.4 124.4,119.6 160.5,119.9 196.7,112.9 232.9,111.3 269.1,99.9 305.3,100.0 341.5,91.2 377.6,92.3 413.8,82.0 450.0,75.8 450.0,30.5 413.8,47.9 377.6,59.9 341.5,66.4 305.3,72.3 269.1,79.6 232.9,87.4 196.7,92.1 160.5,104.4 124.4,105.3 88.2,119.0 52.0,121.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,184.4 124.4,119.6 160.5,119.9 196.7,112.9 232.9,111.3 269.1,99.9 305.3,100.0 341.5,91.2 377.6,92.3 413.8,82.0 450.0,75.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,131.3 88.2,127.5 124.4,115.7 160.5,111.0 196.7,105.4 232.9,107.3 269.1,93.1 305.3,87.2 341.5,82.5 377.6,79.4 413.8,71.3 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.6 88.2,121.1 124.4,109.8 160.5,109.4 196.7,93.8 232.9,95.1 269.1,90.1 305.3,83.0 341.5,79.5 377.6,71.6 413.8,62.6 450.0,55.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.9 88.2,119.7 124.4,109.5 160.5,104.0 196.7,93.4 232.9,92.1 269.1,83.6 305.3,77.4 341.5,70.9 377.6,65.4 413.8,58.2 450.0,52.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.4 88.2,119.0 124.4,105.3 160.5,104.4 196.7,92.1 232.9,87.4 269.1,79.6 305.3,72.3 341.5,66.4 377.6,59.9 413.8,47.9 450.0,30.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.82 ns | 120 ns | 196 ns | 162 ns | 197 ns |
| D38 | 4.36 ns | 150 ns | 196 ns | 198 ns | 199 ns |
| D57 | 276 ns | 303 ns | 425 ns | 440 ns | 526 ns |
| D76 | 271 ns | 407 ns | 436 ns | 571 ns | 557 ns |
| D115 | 412 ns | 555 ns | 1.01 µs | 1.04 µs | 1.06 µs |
| D153 | 491 ns | 468 ns | 995 ns | 1.11 µs | 1.41 µs |
| D230 | 989 ns | 1.2 µs | 1.34 µs | 1.88 µs | 2.21 µs |
| D307 | 888 ns | 1.72 µs | 2.04 µs | 2.66 µs | 3.44 µs |
| D462 | 1.52 µs | 2 µs | 2.35 µs | 3.61 µs | 4.5 µs |
| D616 | 1.43 µs | 2.38 µs | 3.6 µs | 4.85 µs | 6.42 µs |
| D924 | 2.56 µs | 3.91 µs | 5.85 µs | 7.09 µs | 12.4 µs |
| D1232 | 3.49 µs | 5.41 µs | 8.71 µs | 9.59 µs | 32.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,184.4 124.4,112.4 160.5,112.7 196.7,105.4 232.9,102.3 269.1,90.2 305.3,92.1 341.5,82.7 377.6,83.8 413.8,73.6 450.0,68.3 450.0,29.6 413.8,46.2 377.6,57.7 341.5,63.9 305.3,68.5 269.1,76.2 232.9,84.0 196.7,88.9 160.5,100.2 124.4,101.2 88.2,118.0 52.0,118.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,184.4 124.4,112.4 160.5,112.7 196.7,105.4 232.9,102.3 269.1,90.2 305.3,92.1 341.5,82.7 377.6,83.8 413.8,73.6 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,126.8 88.2,123.0 124.4,110.8 160.5,105.6 196.7,100.2 232.9,103.2 269.1,86.8 305.3,80.6 341.5,78.0 377.6,74.9 413.8,66.3 450.0,60.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.3 88.2,118.3 124.4,104.9 160.5,104.4 196.7,89.9 232.9,90.1 269.1,85.0 305.3,77.7 341.5,75.1 377.6,67.8 413.8,59.3 450.0,52.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.6 88.2,118.1 124.4,104.2 160.5,99.7 196.7,89.2 232.9,88.2 269.1,79.0 305.3,73.0 341.5,67.7 377.6,62.6 413.8,56.0 450.0,50.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.2 88.2,118.0 124.4,101.2 160.5,100.2 196.7,88.9 232.9,84.0 269.1,76.2 305.3,68.5 341.5,63.9 377.6,57.7 413.8,46.2 450.0,29.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
