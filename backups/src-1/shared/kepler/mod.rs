/* References
    https://en.wikipedia.org/wiki/Semi-major_and_semi-minor_axes
    https://www.storyofmathematics.com/conic-sections/
    https://oer.pressbooks.pub/lynnanegeorge/chapter/chapter-6-keplers-prediction-problem/

    Calculations in radians
        (Non) means non fact, which is calculated

    True Anomaly
        /*
            /// (Non) The angular position of a body in the elliptic kepler orbit
            // fn eccentric_anomaly(&mut self) -> Anomaly;
            /// (Non) The angle between the current position from perihelion
            
            /// https://www-mars.lmd.jussieu.fr/mars/time/martian_time.html
            ///
            /// Solving Kepler's Equation
            /// https://en.wikipedia.org/wiki/Kepler%27s_equation
            /// En+1 = En - ( (En - e * En.sin() - M(t) ) / (1 - e * En.cos()) );
            /// = En + En+1
            ///
            /// zdz > 10.0 for more precision
            /// En = zx0
            /// M(t) = xref
            /// En+1 = zdx
            /// zanom = Initial Mean Motion
            ///
            /// Solving Eccentric Anomaly
            /// https://en.wikipedia.org/wiki/True_anomaly
            /// v = 2 * ( ((1 + e) / (1 - e)).sqrt() * (E / 2).tan() ).atan()
            /// v = theta
            /// e = Orbital Eccentricty
        */
    Solar Longitude
        /*
            /// computes ls from day given
            /// https://squarewidget.com/keplers-equation/
            /// https://squarewidget.com/solar-coordinates/
            /// Find the true position of the planet by it's anomaly
            /// M = l - u
            /// l = Mean Longitude
            /// u = Longitude of the pericenter
            /// https://en.wikipedia.org/wiki/Mean_anomaly
        */
*/

pub mod seasons;
pub mod orbit;
pub mod datetime;
pub mod trig;
pub mod body;
pub mod timezone;
