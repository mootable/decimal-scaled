# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 |
| :-- | --: |
| D38 | 7.94 µs |
| D57 | 993 ns |
| D76 | 6.76 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="251.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><polygon points="52.0,20.0 251.0,110.3 450.0,27.0 450.0,27.0 251.0,110.3 52.0,20.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,20.0 251.0,110.3 450.0,27.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | max |
| :-- | --: | --: |
| D38 | 7.9 µs | · |
| D57 | 799 ns | · |
| D76 | 7.13 µs | 6.73 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="251.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><polyline points="52.0,20.2 251.0,119.8 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 |
| :-- | --: |
| D38 | 3.5 µs |
| D76 | 5.24 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><polygon points="52.0,101.1 450.0,66.1 450.0,66.1 52.0,101.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,101.1 450.0,66.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ⅓ | ⅔ | max |
| :-- | --: | --: | --: | --: |
| D18 | 2.81 µs | · | · | · |
| D38 | 2.52 µs | 3.46 µs | · | · |
| D57 | 1.9 µs | 4.84 µs | · | · |
| D76 | 2.97 µs | 4.04 µs | 3.64 µs | 9.83 µs |
| D115 | 18.4 µs | · | · | · |
| D153 | 3.13 µs | · | · | · |
| D230 | 10.1 µs | 17.7 µs | 39.9 µs | · |
| D307 | 60 µs | · | · | · |
| D462 | 2.4 µs | 15.2 µs | · | · |
| D616 | 32.8 µs | · | · | · |
| D1232 | 2.86 µs | · | · | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="91.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="131.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="171.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="211.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="251.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="290.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="330.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="370.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="410.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,165.1 91.8,169.9 131.6,182.0 171.4,162.8 211.2,83.6 251.0,160.5 290.8,109.6 330.6,32.2 370.4,171.9 410.2,58.4 450.0,164.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="91.8,156.1 131.6,141.5 171.4,149.3 290.8,85.3 370.4,91.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="171.4,153.9 290.8,49.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.75 µs | · | · | · | · |
| D38 | 2.36 µs | 3.3 µs | · | · | · |
| D57 | 1.96 µs | 4.58 µs | · | · | · |
| D76 | 3.09 µs | 3.86 µs | 3.42 µs | 7.07 µs | 9.56 µs |
| D115 | 12.2 µs | 18 µs | · | · | · |
| D153 | 3.13 µs | 29 µs | · | · | · |
| D230 | 10.2 µs | 16.9 µs | 39.7 µs | · | · |
| D307 | 11.9 µs | 57.1 µs | · | · | · |
| D462 | 2.35 µs | 15.2 µs | 218 µs | · | · |
| D616 | 31.9 µs | · | · | · | · |
| D1232 | 3.02 µs | · | · | · | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="91.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="131.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="171.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="211.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="251.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="290.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="330.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="370.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="410.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,180.7 91.8,185.2 131.6,190.6 171.4,177.3 211.2,137.5 251.0,177.0 290.8,142.7 330.6,138.2 370.4,185.2 410.2,109.8 450.0,178.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="91.8,175.4 131.6,165.9 171.4,170.9 211.2,126.3 251.0,112.5 290.8,128.1 330.6,92.9 370.4,131.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="171.4,174.4 290.8,103.4 370.4,54.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 |
| :-- | --: |
| D76 | 5.4 µs |

<figure>

<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ½ | max |
| :-- | --: | --: | --: |
| D38 | 5.52 µs | · | · |
| D57 | 2.52 µs | · | · |
| D76 | 5.01 µs | 4.58 µs | 11.9 µs |
| D230 | 44.9 µs | · | · |
| D307 | 66 µs | · | · |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="151.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="251.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="350.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><polyline points="52.0,135.8 151.5,169.8 251.0,140.0 350.5,44.8 450.0,28.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
