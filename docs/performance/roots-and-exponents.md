# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.73 µs | 5.14 µs | 5.15 µs | 8.7 µs | 8.7 µs |
| D38 | 5.08 µs | 5.15 µs | 3.63 µs | 10.9 µs | 10.9 µs |
| D57 | 355 ns | 600 ns | 717 ns | 1.04 µs | 1.14 µs |
| D76 | 540 ns | 789 ns | 1.12 µs | 1.36 µs | 1.92 µs |
| D115 | 507 ns | 2.09 µs | 2.05 µs | 3.54 µs | 5.37 µs |
| D153 | 526 ns | 2.66 µs | 3.58 µs | 5.76 µs | 6.1 µs |
| D230 | 400 ns | 4.07 µs | 7.97 µs | 10.1 µs | 14.7 µs |
| D307 | 409 ns | 6.83 µs | 11.2 µs | 16.7 µs | 22.6 µs |
| D462 | 467 ns | 10.3 µs | 24.4 µs | 35.8 µs | 47.8 µs |
| D616 | 527 ns | 19 µs | 32.9 µs | 57.6 µs | 84.9 µs |
| D924 | 705 ns | 32.7 µs | 81.9 µs | 121 µs | 179 µs |
| D1232 | 457 ns | 49.4 µs | 118 µs | 228 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,122.1 88.2,124.7 124.4,182.5 160.5,173.4 196.7,174.7 232.9,174.0 269.1,179.9 305.3,179.4 341.5,176.5 377.6,173.9 413.8,167.6 450.0,177.0 450.0,33.1 413.8,47.3 377.6,63.6 341.5,76.0 305.3,92.3 269.1,101.6 232.9,120.7 196.7,123.5 160.5,145.8 124.4,157.1 88.2,108.1 52.0,113.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,122.1 88.2,124.7 124.4,182.5 160.5,173.4 196.7,174.7 232.9,174.0 269.1,179.9 305.3,179.4 341.5,176.5 377.6,173.9 413.8,167.6 450.0,177.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.5 88.2,124.4 124.4,171.1 160.5,165.1 196.7,144.0 232.9,138.8 269.1,129.5 305.3,118.3 341.5,109.3 377.6,96.1 413.8,84.3 450.0,75.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.4 88.2,132.0 124.4,167.2 160.5,157.4 196.7,144.4 232.9,132.3 269.1,114.9 305.3,107.5 341.5,90.6 377.6,84.1 413.8,64.3 450.0,56.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,108.1 124.4,159.1 160.5,153.3 196.7,132.6 232.9,122.0 269.1,109.9 305.3,98.9 341.5,82.3 377.6,72.0 413.8,55.9 450.0,42.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,108.1 124.4,157.1 160.5,145.8 196.7,123.5 232.9,120.7 269.1,101.6 305.3,92.3 341.5,76.0 377.6,63.6 413.8,47.3 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.17 ns | 4.41 µs | 6.24 µs | 7.32 µs | 7.87 µs |
| D38 | 1.56 ns | 6.24 µs | 4.83 µs | 9.5 µs | 10.7 µs |
| D57 | 2.81 ns | 3.47 µs | 4.1 µs | 5.79 µs | 8.68 µs |
| D76 | 3.23 ns | 5.49 µs | 7.26 µs | 8.84 µs | 12.1 µs |
| D115 | 17.4 ns | 6.82 µs | 12 µs | 18.6 µs | 23 µs |
| D153 | 22.4 ns | 6.92 µs | 15.1 µs | 21.7 µs | 31.7 µs |
| D230 | 57.5 ns | 13.3 µs | 21.6 µs | 41.8 µs | 81.7 µs |
| D307 | 104 ns | 14.8 µs | 34.4 µs | 76.3 µs | 122 µs |
| D462 | 163 ns | 23.2 µs | 78.1 µs | 161 µs | 225 µs |
| D616 | 172 ns | 37.7 µs | 121 µs | 261 µs | 438 µs |
| D924 | 180 ns | 79 µs | 258 µs | 545 µs | 880 µs |
| D1232 | 203 ns | 138 µs | 348 µs | 905 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,204.5 124.4,197.2 160.5,195.5 196.7,174.5 232.9,171.4 269.1,159.7 305.3,152.3 341.5,146.8 377.6,146.1 413.8,145.6 450.0,144.1 450.0,25.6 413.8,40.2 377.6,48.8 341.5,57.1 305.3,64.7 269.1,69.6 232.9,81.4 196.7,85.4 160.5,93.4 124.4,97.5 88.2,94.9 52.0,98.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,204.5 124.4,197.2 160.5,195.5 196.7,174.5 232.9,171.4 269.1,159.7 305.3,152.3 341.5,146.8 377.6,146.1 413.8,145.6 450.0,144.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,105.9 88.2,101.6 124.4,108.9 160.5,103.2 196.7,100.5 232.9,100.3 269.1,92.1 305.3,90.8 341.5,85.3 377.6,79.2 413.8,70.1 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.6 88.2,104.7 124.4,106.8 160.5,99.7 196.7,93.4 232.9,90.6 269.1,86.1 305.3,80.4 341.5,70.2 377.6,64.8 413.8,55.4 450.0,51.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.6 88.2,96.3 124.4,102.5 160.5,97.2 196.7,88.0 232.9,86.1 269.1,78.0 305.3,70.5 341.5,61.2 377.6,55.3 413.8,46.1 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.7 88.2,94.9 124.4,97.5 160.5,93.4 196.7,85.4 232.9,81.4 269.1,69.6 305.3,64.7 341.5,57.1 377.6,48.8 413.8,40.2 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.3 ns | 40.8 ns | 40.8 ns | 40.3 ns | 40.1 ns |
| D38 | 16.1 ns | 41.4 ns | 70 ns | 75.4 ns | 106 ns |
| D57 | 16.6 ns | 39.9 ns | 67.9 ns | 661 ns | 709 ns |
| D76 | 17.3 ns | 70.2 ns | 655 ns | 717 ns | 915 ns |
| D115 | 22.4 ns | 79.2 ns | 490 ns | 1.1 µs | 1.36 µs |
| D153 | 25.5 ns | 702 ns | 1.06 µs | 1.44 µs | 1.79 µs |
| D230 | 28.2 ns | 641 ns | 1.45 µs | 2.36 µs | 3.19 µs |
| D307 | 43.8 ns | 1.07 µs | 2.12 µs | 3.28 µs | 5.59 µs |
| D462 | 62.9 ns | 1.41 µs | 3.67 µs | 6.41 µs | 9.73 µs |
| D616 | 84.1 ns | 2.42 µs | 5.56 µs | 10.9 µs | 15.4 µs |
| D924 | 111 ns | 3.81 µs | 10.9 µs | 22.9 µs | 28.2 µs |
| D1232 | 54.5 ns | 6.24 µs | 16.1 µs | 27.5 µs | 50.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.1 88.2,199.6 124.4,199.1 160.5,198.1 196.7,192.4 232.9,189.7 269.1,187.5 305.3,177.9 341.5,170.1 377.6,163.7 413.8,157.7 450.0,173.2 450.0,24.7 413.8,37.5 377.6,50.7 341.5,60.6 305.3,72.6 269.1,84.8 232.9,97.4 196.7,103.4 160.5,111.9 124.4,117.5 88.2,158.6 52.0,179.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.1 88.2,199.6 124.4,199.1 160.5,198.1 196.7,192.4 232.9,189.7 269.1,187.5 305.3,177.9 341.5,170.1 377.6,163.7 413.8,157.7 450.0,173.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.5 88.2,179.1 124.4,180.0 160.5,167.7 196.7,165.1 232.9,117.7 269.1,119.7 305.3,108.5 341.5,102.6 377.6,90.8 413.8,81.0 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,167.7 124.4,168.4 160.5,119.2 196.7,125.5 232.9,108.7 269.1,101.9 305.3,93.7 341.5,81.7 377.6,72.8 413.8,58.1 450.0,49.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,166.1 124.4,119.0 160.5,117.2 196.7,108.0 232.9,102.1 269.1,91.3 305.3,84.2 341.5,69.7 377.6,58.0 413.8,42.0 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.9 88.2,158.6 124.4,117.5 160.5,111.9 196.7,103.4 232.9,97.4 269.1,84.8 305.3,72.6 341.5,60.6 377.6,50.7 413.8,37.5 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 274 ns | 363 ns | 397 ns | 400 ns |
| D38 | 2.18 ns | 362 ns | 356 ns | 407 ns | 410 ns |
| D57 | 267 ns | 440 ns | 434 ns | 477 ns | 545 ns |
| D76 | 270 ns | 446 ns | 464 ns | 543 ns | 654 ns |
| D115 | 274 ns | 474 ns | 498 ns | 1.03 µs | 1.05 µs |
| D153 | 289 ns | 467 ns | 575 ns | 932 ns | 1.11 µs |
| D230 | 548 ns | 710 ns | 945 ns | 1.21 µs | 1.74 µs |
| D307 | 761 ns | 648 ns | 1.02 µs | 1.22 µs | 10 µs |
| D462 | 1.25 µs | 3.13 µs | 3.34 µs | 4.13 µs | 4.73 µs |
| D616 | 1.54 µs | 1.5 µs | 1.4 µs | 2.57 µs | 3.56 µs |
| D924 | 1.85 µs | 1.79 µs | 2.72 µs | 3.31 µs | 4.26 µs |
| D1232 | 1.59 µs | 3.06 µs | 3.36 µs | 4.62 µs | 6.52 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,196.4 124.4,112.9 160.5,112.8 196.7,112.5 232.9,111.6 269.1,100.4 305.3,94.8 341.5,86.1 377.6,82.5 413.8,79.3 450.0,81.9 450.0,57.4 413.8,64.8 377.6,67.9 341.5,63.0 305.3,50.0 269.1,80.3 232.9,88.2 196.7,89.2 160.5,97.4 124.4,100.5 88.2,105.5 52.0,105.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,196.4 124.4,112.9 160.5,112.8 196.7,112.5 232.9,111.6 269.1,100.4 305.3,94.8 341.5,86.1 377.6,82.5 413.8,79.3 450.0,81.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.5 88.2,107.6 124.4,104.3 160.5,104.0 196.7,103.0 232.9,103.2 269.1,95.9 305.3,97.5 341.5,70.2 377.6,83.0 413.8,79.9 450.0,70.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,107.9 124.4,104.5 160.5,103.3 196.7,102.1 232.9,99.6 269.1,91.0 305.3,89.6 341.5,69.1 377.6,84.1 413.8,72.6 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.0 88.2,105.6 124.4,102.9 160.5,100.6 196.7,89.5 232.9,91.2 269.1,86.7 305.3,86.6 341.5,65.3 377.6,73.6 413.8,69.2 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.9 88.2,105.5 124.4,100.5 160.5,97.4 196.7,89.2 232.9,88.2 269.1,80.3 305.3,50.0 341.5,63.0 377.6,67.9 413.8,64.8 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.15 µs | 8.74 µs | 11.6 µs | 13.6 µs | 14.9 µs |
| D38 | 7.7 µs | 11.6 µs | 9.23 µs | 18.4 µs | 21.2 µs |
| D57 | 4.36 µs | 4.02 µs | 4.27 µs | 4.9 µs | 4.55 µs |
| D76 | 4.37 µs | 4.31 µs | 4.85 µs | 4.53 µs | 5.29 µs |
| D115 | 7.52 µs | 9.04 µs | 7.72 µs | 10.1 µs | 10.7 µs |
| D153 | 7.73 µs | 8.39 µs | 8.93 µs | 9.75 µs | 9.5 µs |
| D230 | 10.7 µs | 12.6 µs | 13.6 µs | 14.6 µs | 17.6 µs |
| D307 | 16.3 µs | 18.4 µs | 21.3 µs | 23.5 µs | 28.3 µs |
| D462 | 16.5 µs | 21.5 µs | 23.7 µs | 29.3 µs | 31.2 µs |
| D616 | 28.2 µs | 42.4 µs | 44.9 µs | 60.9 µs | 78.4 µs |
| D924 | 39.8 µs | 67.7 µs | 94.4 µs | 122 µs | 148 µs |
| D1232 | 30.1 µs | 113 µs | 131 µs | 201 µs | 273 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,149.2 88.2,150.9 124.4,167.4 160.5,167.3 196.7,151.6 232.9,150.8 269.1,141.4 305.3,129.2 341.5,128.9 377.6,113.3 413.8,103.3 450.0,111.4 450.0,47.6 413.8,65.3 377.6,83.7 341.5,110.4 305.3,113.2 269.1,126.9 232.9,144.8 196.7,141.3 160.5,161.8 124.4,166.1 88.2,121.6 52.0,131.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,149.2 88.2,150.9 124.4,167.4 160.5,167.3 196.7,151.6 232.9,150.8 269.1,141.4 305.3,129.2 341.5,128.9 377.6,113.3 413.8,103.3 450.0,111.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,147.2 88.2,139.0 124.4,169.7 160.5,167.7 196.7,146.2 232.9,148.4 269.1,136.6 305.3,125.6 341.5,121.2 377.6,101.5 413.8,88.0 450.0,73.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,139.0 88.2,145.7 124.4,168.0 160.5,164.3 196.7,150.8 232.9,146.6 269.1,134.4 305.3,121.5 341.5,118.4 377.6,99.9 413.8,78.3 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,125.7 124.4,164.0 160.5,166.3 196.7,142.9 232.9,144.1 269.1,132.4 305.3,118.6 341.5,112.3 377.6,91.0 413.8,70.9 450.0,56.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,121.6 124.4,166.1 160.5,161.8 196.7,141.3 232.9,144.8 269.1,126.9 305.3,113.2 341.5,110.4 377.6,83.7 413.8,65.3 450.0,47.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.14 ns | 4.86 µs | 6.88 µs | 8.01 µs | 8.6 µs |
| D38 | 6.54 ns | 6.88 µs | 5.44 µs | 10.2 µs | 11.5 µs |
| D57 | 62.5 ns | 3.91 µs | 4.25 µs | 4.83 µs | 4.9 µs |
| D76 | 78.4 ns | 3.86 µs | 4.83 µs | 4.89 µs | 5.85 µs |
| D115 | 138 ns | 8.67 µs | 8.12 µs | 10.9 µs | 11.5 µs |
| D153 | 173 ns | 8.4 µs | 9.51 µs | 10.7 µs | 10.9 µs |
| D230 | 331 ns | 13.2 µs | 14.6 µs | 17.7 µs | 22.1 µs |
| D307 | 435 ns | 18.9 µs | 21.3 µs | 27.8 µs | 34.4 µs |
| D462 | 712 ns | 76.6 µs | 135 µs | 219 µs | 257 µs |
| D616 | 831 ns | 191 µs | 324 µs | 349 µs | 567 µs |
| D924 | 950 ns | 422 µs | 451 µs | 774 µs | 1.58 ms |
| D1232 | 788 ns | 831 µs | 675 µs | 2.11 ms | 2.99 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,186.7 124.4,158.7 160.5,155.9 196.7,148.9 232.9,146.0 269.1,138.0 305.3,134.6 341.5,128.5 377.6,126.6 413.8,124.9 450.0,127.2 450.0,25.0 413.8,32.9 377.6,45.6 341.5,55.4 305.3,80.4 269.1,85.9 232.9,94.7 196.7,94.0 160.5,102.4 124.4,104.6 88.2,94.0 52.0,97.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,186.7 124.4,158.7 160.5,155.9 196.7,148.9 232.9,146.0 269.1,138.0 305.3,134.6 341.5,128.5 377.6,126.6 413.8,124.9 450.0,127.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.7 88.2,100.4 124.4,107.4 160.5,107.5 196.7,97.5 232.9,97.9 269.1,92.3 305.3,87.8 341.5,70.5 377.6,59.1 413.8,49.3 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.4 88.2,103.3 124.4,106.3 160.5,104.7 196.7,98.3 232.9,96.3 269.1,91.0 305.3,86.3 341.5,63.4 377.6,52.5 413.8,48.5 450.0,43.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.5 88.2,95.4 124.4,104.8 160.5,104.6 196.7,94.6 232.9,94.8 269.1,88.6 305.3,83.0 341.5,57.4 377.6,51.6 413.8,41.7 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.6 88.2,94.0 124.4,104.6 160.5,102.4 196.7,94.0 232.9,94.7 269.1,85.9 305.3,80.4 341.5,55.4 377.6,45.6 413.8,32.9 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.66 ns | 14.6 ns | 19.1 ns | 30.4 ns | 30.6 ns |
| D38 | 7.62 ns | 20.3 ns | 35 ns | 1.64 µs | 3.19 µs |
| D57 | 210 ns | 207 ns | 488 ns | 675 ns | 765 ns |
| D76 | 212 ns | 269 ns | 646 ns | 843 ns | 1.06 µs |
| D115 | 116 ns | 598 ns | 769 ns | 1.52 µs | 1.66 µs |
| D153 | 119 ns | 1.12 µs | 1.61 µs | 1.91 µs | 2.25 µs |
| D230 | 165 ns | 1.71 µs | 2.4 µs | 3.5 µs | 4.2 µs |
| D307 | 157 ns | 2.34 µs | 3.66 µs | 4.64 µs | 7.27 µs |
| D462 | 187 ns | 3.81 µs | 5.89 µs | 9.85 µs | 11.9 µs |
| D616 | 250 ns | 6.46 µs | 9.02 µs | 14.3 µs | 20.5 µs |
| D924 | 220 ns | 11.1 µs | 16.9 µs | 26.1 µs | 34.4 µs |
| D1232 | 144 ns | 16.7 µs | 22.4 µs | 39.8 µs | 62.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.1 88.2,174.7 124.4,117.1 160.5,116.9 196.7,127.4 232.9,126.9 269.1,121.3 305.3,122.2 341.5,119.1 377.6,114.1 413.8,116.3 450.0,123.6 450.0,18.2 413.8,28.6 377.6,37.6 341.5,47.0 305.3,55.5 269.1,65.1 232.9,75.9 196.7,81.2 160.5,89.0 124.4,94.6 88.2,69.9 52.0,150.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.1 88.2,174.7 124.4,117.1 160.5,116.9 196.7,127.4 232.9,126.9 269.1,121.3 305.3,122.2 341.5,119.1 377.6,114.1 413.8,116.3 450.0,123.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.5 88.2,157.7 124.4,117.4 160.5,112.8 196.7,98.9 232.9,88.0 269.1,80.7 305.3,75.2 341.5,66.7 377.6,57.6 413.8,48.1 450.0,41.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,148.2 124.4,102.5 160.5,97.6 196.7,94.6 232.9,81.7 269.1,74.8 305.3,67.4 341.5,59.2 377.6,51.8 413.8,40.9 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.7 88.2,81.4 124.4,96.8 160.5,93.0 196.7,82.7 232.9,78.8 269.1,68.2 305.3,63.3 341.5,50.3 377.6,43.8 413.8,33.3 450.0,26.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.5 88.2,69.9 124.4,94.6 160.5,89.0 196.7,81.2 232.9,75.9 269.1,65.1 305.3,55.5 341.5,47.0 377.6,37.6 413.8,28.6 450.0,18.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
