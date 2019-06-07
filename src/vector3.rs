use wasm_bindgen::prelude::*;

use super::common::*;
use super::matrix3::*;
use super::matrix4::*;
use super::quaternion::*;

#[wasm_bindgen]
pub struct Vector3(pub f32, pub f32, pub f32);

#[wasm_bindgen]
impl Vector3 {
    #[wasm_bindgen(getter)]
    pub fn elements(&self) -> Box<[f32]> {
        Box::new([self.0, self.1, self.2])
    }

    /**
     * 3 Dimensional Vector
     * @module vec3
     */

    /**
     * Creates a new, empty vec3
     *
     * @returns {vec3} a new 3D vector
     */
    pub fn create() -> Vector3 {
        Vector3(0., 0., 0.)
    }

    /**
     * Creates a new vec3 initialized with values from an existing vector
     *
     * @param {vec3} a vector to clone
     * @returns {vec3} a new 3D vector
     */
    pub fn clone(a: &Vector3) -> Vector3 {
        Vector3(a.0, a.1, a.2)
    }

    /**
     * Calculates the length of a vec3
     *
     * @param {vec3} a vector to calculate length of
     * @returns {Number} length of a
     */
    pub fn length(a: &Vector3) -> f32 {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }

    /**
     * Creates a new vec3 initialized with the given values
     *
     * @param {Number} x X component
     * @param {Number} y Y component
     * @param {Number} z Z component
     * @returns {vec3} a new 3D vector
     */
    pub fn fromValues(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3(x, y, z)
    }

    /**
     * Copy the values from one vec3 to another
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the source vector
     * @returns {vec3} out
     */
    pub fn copy(out: &mut Vector3, a: &Vector3) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = a.2;
    }

    /**
     * Set the components of a vec3 to the given values
     *
     * @param {vec3} out the receiving vector
     * @param {Number} x X component
     * @param {Number} y Y component
     * @param {Number} z Z component
     * @returns {vec3} out
     */
    pub fn set(out: &mut Vector3, x: f32, y: f32, z: f32) {
        out.0 = x;
        out.1 = y;
        out.2 = z;
    }

    /**
     * Adds two vec3"s
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @returns {vec3} out
     */
    pub fn add(out: &mut Vector3, a: &Vector3, b: &Vector3) {
        out.0 = a.0 + b.0;
        out.1 = a.1 + b.1;
        out.2 = a.2 + b.2;
    }

    /**
     * Subtracts vector b from vector a
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @returns {vec3} out
     */
    pub fn subtract(out: &mut Vector3, a: &Vector3, b: &Vector3) {
        out.0 = a.0 - b.0;
        out.1 = a.1 - b.1;
        out.2 = a.2 - b.2;
    }

    /**
     * Multiplies two vec3"s
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @returns {vec3} out
     */
    pub fn multiply(out: &mut Vector3, a: &Vector3, b: &Vector3) {
        out.0 = a.0 * b.0;
        out.1 = a.1 * b.1;
        out.2 = a.2 * b.2;
    }

    /**
     * Divides two vec3"s
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @returns {vec3} out
     */
    pub fn divide(out: &mut Vector3, a: &Vector3, b: &Vector3) {
        out.0 = a.0 / b.0;
        out.1 = a.1 / b.1;
        out.2 = a.2 / b.2;
    }

    /**
     * f32::ceil the components of a vec3
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a vector to ceil
     * @returns {vec3} out
     */
    pub fn ceil(out: &mut Vector3, a: &Vector3) {
        out.0 = f32::ceil(a.0);
        out.1 = f32::ceil(a.1);
        out.2 = f32::ceil(a.2);
    }

    /**
     * f32::floor the components of a vec3
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a vector to floor
     * @returns {vec3} out
     */
    pub fn floor(out: &mut Vector3, a: &Vector3) {
        out.0 = f32::floor(a.0);
        out.1 = f32::floor(a.1);
        out.2 = f32::floor(a.2);
    }

    /**
     * Returns the minimum of two vec3"s
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @returns {vec3} out
     */
    pub fn min(out: &mut Vector3, a: &Vector3, b: &Vector3) {
        out.0 = f32::min(a.0, b.0);
        out.1 = f32::min(a.1, b.1);
        out.2 = f32::min(a.2, b.2);
    }

    /**
     * Returns the maximum of two vec3"s
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @returns {vec3} out
     */
    pub fn max(out: &mut Vector3, a: &Vector3, b: &Vector3) {
        out.0 = f32::max(a.0, b.0);
        out.1 = f32::max(a.1, b.1);
        out.2 = f32::max(a.2, b.2);
    }

    /**
     * f32::round the components of a vec3
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a vector to round
     * @returns {vec3} out
     */
    pub fn round(out: &mut Vector3, a: &Vector3) {
        out.0 = f32::round(a.0);
        out.1 = f32::round(a.1);
        out.2 = f32::round(a.2);
    }

    /**
     * Scales a vec3 by a scalar number
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the vector to scale
     * @param {Number} b amount to scale the vector by
     * @returns {vec3} out
     */
    pub fn scale(out: &mut Vector3, a: &Vector3, b: f32) {
        out.0 = a.0 * b;
        out.1 = a.1 * b;
        out.2 = a.2 * b;
    }

    /**
     * Adds two vec3"s after scaling the second operand by a scalar value
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @param {Number} scale the amount to scale b by before adding
     * @returns {vec3} out
     */
    pub fn scaleAndAdd(out: &mut Vector3, a: &Vector3, b: &Vector3, scale: f32) {
        out.0 = a.0 + (b.0 * scale);
        out.1 = a.1 + (b.1 * scale);
        out.2 = a.2 + (b.2 * scale);
    }

    /**
     * Calculates the euclidian distance between two vec3"s
     *
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @returns {Number} distance between a and b
     */
    pub fn distance(a: &Vector3, b: &Vector3) -> f32 {
        let x = b.0 - a.0;
        let y = b.1 - a.1;
        let z = b.2 - a.2;
        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }

    /**
     * Calculates the squared euclidian distance between two vec3"s
     *
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @returns {Number} squared distance between a and b
     */
    pub fn squaredDistance(a: &Vector3, b: &Vector3) -> f32 {
        let x = b.0 - a.0;
        let y = b.1 - a.1;
        let z = b.2 - a.2;
        x * x + y * y + z * z
    }

    /**
     * Calculates the squared length of a vec3
     *
     * @param {vec3} a vector to calculate squared length of
     * @returns {Number} squared length of a
     */
    pub fn squaredLength(a: &Vector3) -> f32 {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        x * x + y * y + z * z
    }

    /**
     * Negates the components of a vec3
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a vector to negate
     * @returns {vec3} out
     */
    pub fn negate(out: &mut Vector3, a: &Vector3) {
        out.0 = -a.0;
        out.1 = -a.1;
        out.2 = -a.2;
    }

    /**
     * Returns the inverse of the components of a vec3
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a vector to invert
     * @returns {vec3} out
     */
    pub fn inverse(out: &mut Vector3, a: &Vector3) {
        out.0 = 1.0 / a.0;
        out.1 = 1.0 / a.1;
        out.2 = 1.0 / a.2;
    }

    /**
     * Normalize a vec3
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a vector to normalize
     * @returns {vec3} out
     */
    pub fn normalize(out: &mut Vector3, a: &Vector3) {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let mut len = x * x + y * y + z * z;
        if (len > EPSILON) {
            //TODO: evaluate use of glm_invsqrt here?
            len = 1. / f32::sqrt(len);
        }
        out.0 = a.0 * len;
        out.1 = a.1 * len;
        out.2 = a.2 * len;
    }

    /**
     * Calculates the dot product of two vec3"s
     *
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @returns {Number} dot product of a and b
     */
    pub fn dot(a: &Vector3, b: &Vector3) -> f32 {
        return a.0 * b.0 + a.1 * b.1 + a.2 * b.2;
    }

    /**
     * Computes the cross product of two vec3"s
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @returns {vec3} out
     */
    pub fn cross(out: &mut Vector3, a: &Vector3, b: &Vector3) {
        let ax = a.0;
        let ay = a.1;
        let az = a.2;
        let bx = b.0;
        let by = b.1;
        let bz = b.2;

        out.0 = ay * bz - az * by;
        out.1 = az * bx - ax * bz;
        out.2 = ax * by - ay * bx;
    }

    /**
     * Performs a linear interpolation between two vec3"s
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @param {Number} t interpolation amount, in the range [0-1], between the two inputs
     * @returns {vec3} out
     */
    pub fn lerp(out: &mut Vector3, a: &Vector3, b: &Vector3, t: f32) {
        let ax = a.0;
        let ay = a.1;
        let az = a.2;
        out.0 = ax + t * (b.0 - ax);
        out.1 = ay + t * (b.1 - ay);
        out.2 = az + t * (b.2 - az);
    }

    /**
     * Performs a hermite interpolation with two control points
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @param {vec3} c the third operand
     * @param {vec3} d the fourth operand
     * @param {Number} t interpolation amount, in the range [0-1], between the two inputs
     * @returns {vec3} out
     */
    pub fn hermite(out: &mut Vector3, a: &Vector3, b: &Vector3, c: &Vector3, d: &Vector3, t: f32) {
        let factorTimes2 = t * t;
        let factor1 = factorTimes2 * (2. * t - 3.) + 1.;
        let factor2 = factorTimes2 * (t - 2.) + t;
        let factor3 = factorTimes2 * (t - 1.);
        let factor4 = factorTimes2 * (3. - 2. * t);

        out.0 = a.0 * factor1 + b.0 * factor2 + c.0 * factor3 + d.0 * factor4;
        out.1 = a.1 * factor1 + b.1 * factor2 + c.1 * factor3 + d.1 * factor4;
        out.2 = a.2 * factor1 + b.2 * factor2 + c.2 * factor3 + d.2 * factor4;
    }

    /**
     * Performs a bezier interpolation with two control points
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the first operand
     * @param {vec3} b the second operand
     * @param {vec3} c the third operand
     * @param {vec3} d the fourth operand
     * @param {Number} t interpolation amount, in the range [0-1], between the two inputs
     * @returns {vec3} out
     */
    pub fn bezier(out: &mut Vector3, a: &Vector3, b: &Vector3, c: &Vector3, d: &Vector3, t: f32) {
        let inverseFactor = 1. - t;
        let inverseFactorTimesTwo = inverseFactor * inverseFactor;
        let factorTimes2 = t * t;
        let factor1 = inverseFactorTimesTwo * inverseFactor;
        let factor2 = 3. * t * inverseFactorTimesTwo;
        let factor3 = 3. * factorTimes2 * inverseFactor;
        let factor4 = factorTimes2 * t;

        out.0 = a.0 * factor1 + b.0 * factor2 + c.0 * factor3 + d.0 * factor4;
        out.1 = a.1 * factor1 + b.1 * factor2 + c.1 * factor3 + d.1 * factor4;
        out.2 = a.2 * factor1 + b.2 * factor2 + c.2 * factor3 + d.2 * factor4;
    }

    /**
     * Generates a random vector with the given scale
     *
     * @param {vec3} out the receiving vector
     * @param {Number} [scale] Length of the resulting vector. If ommitted, a unit vector will be returned
     * @returns {vec3} out
     */
    pub fn random(out: &mut Vector3, scale: Option<f32>) {
        let mut s = 1.;
        match scale {
            Some(value) => s = value,
            None => {}
        };
        let scale = s;

        let r = RANDOM() * 2.0 * PI;
        let z = (RANDOM() * 2.0) - 1.0;
        let zScale = f32::sqrt(1.0 - z * z) * scale;

        out.0 = f32::cos(r) * zScale;
        out.1 = f32::sin(r) * zScale;
        out.2 = z * scale;
    }

    /**
     * Transforms the vec3 with a mat4.
     * 4th vector component is implicitly "1"
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the vector to transform
     * @param {mat4} m matrix to transform with
     * @returns {vec3} out
     */
    pub fn transformMat4(out: &mut Vector3, a: &Vector3, m: &Matrix4) {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let mut w = m.3 * x + m.7 * y + m.11 * z + m.15;
        if (w < EPSILON) {
            w = 1.0;
        }
        out.0 = (m.0 * x + m.4 * y + m.8 * z + m.12) / w;
        out.1 = (m.1 * x + m.5 * y + m.9 * z + m.13) / w;
        out.2 = (m.2 * x + m.6 * y + m.10 * z + m.14) / w;
    }

    /**
     * Transforms the vec3 with a mat3.
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the vector to transform
     * @param {mat3} m the 3x3 matrix to transform with
     * @returns {vec3} out
     */
    pub fn transformMat3(out: &mut Vector3, a: &Vector3, m: &Matrix3) {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        out.0 = x * m.0 + y * m.3 + z * m.6;
        out.1 = x * m.1 + y * m.4 + z * m.7;
        out.2 = x * m.2 + y * m.5 + z * m.8;
    }

    /**
     * Transforms the vec3 with a quat
     * Can also be used for dual quaternions. (Multiply it with the real part)
     *
     * @param {vec3} out the receiving vector
     * @param {vec3} a the vector to transform
     * @param {quat} q quaternion to transform with
     * @returns {vec3} out
     */
    pub fn transformQuat(out: &mut Vector3, a: &Vector3, q: &Quaternion) {
        // benchmarks: https://jsperf.com/quaternion-transform-vec3-implementations-fixed
        let qx = q.0;
        let qy = q.1;
        let qz = q.2;
        let qw = q.3;
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let mut uvx = qy * z - qz * y;
        let mut uvy = qz * x - qx * z;
        let mut uvz = qx * y - qy * x;
        let mut uuvx = qy * uvz - qz * uvy;
        let mut uuvy = qz * uvx - qx * uvz;
        let mut uuvz = qx * uvy - qy * uvx;
        // vec3.scale(uv, uv, 2 * w);
        let w2 = qw * 2.;
        uvx *= w2;
        uvy *= w2;
        uvz *= w2;
        uuvx *= 2.;
        uuvy *= 2.;
        uuvz *= 2.;
        // return vec3.add(out, a, vec3.add(out, uv, uuv));
        out.0 = x + uvx + uuvx;
        out.1 = y + uvy + uuvy;
        out.2 = z + uvz + uuvz;
    }

    /**
     * Rotate a 3D vector around the x-axis
     * @param {vec3} out The receiving vec3
     * @param {vec3} a The vec3 point to rotate
     * @param {vec3} b The origin of the rotation
     * @param {Number} c The angle of rotation
     * @returns {vec3} out
     */
    pub fn rotateX(out: &mut Vector3, a: &Vector3, b: &Vector3, c: f32) {
        let p = &mut Vector3::create();
        let r = &mut Vector3::create();
        //Translate point to the origin
        p.0 = a.0 - b.0;
        p.1 = a.1 - b.1;
        p.2 = a.2 - b.2;

        //perform rotation
        r.0 = p.0;
        r.1 = p.1 * f32::cos(c) - p.2 * f32::sin(c);
        r.2 = p.1 * f32::sin(c) + p.2 * f32::cos(c);

        //translate to correct position
        out.0 = r.0 + b.0;
        out.1 = r.1 + b.1;
        out.2 = r.2 + b.2;
    }

    /**
     * Rotate a 3D vector around the y-axis
     * @param {vec3} out The receiving vec3
     * @param {vec3} a The vec3 point to rotate
     * @param {vec3} b The origin of the rotation
     * @param {Number} c The angle of rotation
     * @returns {vec3} out
     */
    pub fn rotateY(out: &mut Vector3, a: &Vector3, b: &Vector3, c: f32) {
        let p = &mut Vector3::create();
        let r = &mut Vector3::create();
        //Translate point to the origin
        p.0 = a.0 - b.0;
        p.1 = a.1 - b.1;
        p.2 = a.2 - b.2;

        //perform rotation
        r.0 = p.2 * f32::sin(c) + p.0 * f32::cos(c);
        r.1 = p.1;
        r.2 = p.2 * f32::cos(c) - p.0 * f32::sin(c);

        //translate to correct position
        out.0 = r.0 + b.0;
        out.1 = r.1 + b.1;
        out.2 = r.2 + b.2;
    }

    /**
     * Rotate a 3D vector around the z-axis
     * @param {vec3} out The receiving vec3
     * @param {vec3} a The vec3 point to rotate
     * @param {vec3} b The origin of the rotation
     * @param {Number} c The angle of rotation
     * @returns {vec3} out
     */
    pub fn rotateZ(out: &mut Vector3, a: &Vector3, b: &Vector3, c: f32) {
        let p = &mut Vector3::create();
        let r = &mut Vector3::create();
        //Translate point to the origin
        p.0 = a.0 - b.0;
        p.1 = a.1 - b.1;
        p.2 = a.2 - b.2;

        //perform rotation
        r.0 = p.0 * f32::cos(c) - p.1 * f32::sin(c);
        r.1 = p.0 * f32::sin(c) + p.1 * f32::cos(c);
        r.2 = p.2;

        //translate to correct position
        out.0 = r.0 + b.0;
        out.1 = r.1 + b.1;
        out.2 = r.2 + b.2;
    }

    /**
     * Get the angle between two 3D vectors
     * @param {vec3} a The first operand
     * @param {vec3} b The second operand
     * @returns {Number} The angle in radians
     */
    pub fn angle(a: &Vector3, b: &Vector3) -> f32 {
        let tempA = &mut Vector3::fromValues(a.0, a.1, a.2);
        let tempB = &mut Vector3::fromValues(b.0, b.1, b.2);

        Vector3::normalize(tempA, &Vector3::clone(tempA));
        Vector3::normalize(tempB, &Vector3::clone(tempB));

        let cosine = Vector3::dot(tempA, tempB);

        if (cosine > 1.0) {
            0.
        } else if (cosine < -1.0) {
            PI
        } else {
            f32::acos(cosine)
        }
    }

    /**
     * Set the components of a vec3 to zero
     *
     * @param {vec3} out the receiving vector
     * @returns {vec3} out
     */
    pub fn zero(out: &mut Vector3) {
        out.0 = 0.0;
        out.1 = 0.0;
        out.2 = 0.0;
    }

    /**
     * Returns a string representation of a vector
     *
     * @param {vec3} a vector to represent as a string
     * @returns {String} string representation of the vector
     */
    // pub fn str(a: &Vector3) {
    //   return "vec3(" + a.0 + ", " + a.1 + ", " + a.2 + ")";
    // }

    /**
     * Returns whether or not the vectors have exactly the same elements in the same position (when compared with ==)
     *
     * @param {vec3} a The first vector.
     * @param {vec3} b The second vector.
     * @returns {Boolean} True if the vectors are equal, false otherwise.
     */
    pub fn exactEquals(a: &Vector3, b: &Vector3) -> bool {
        a.0 == b.0 && a.1 == b.1 && a.2 == b.2
    }

    /**
     * Returns whether or not the vectors have approximately the same elements in the same position.
     *
     * @param {vec3} a The first vector.
     * @param {vec3} b The second vector.
     * @returns {Boolean} True if the vectors are equal, false otherwise.
     */
    pub fn equals(a: &Vector3, b: &Vector3) -> bool {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let b0 = b.0;
        let b1 = b.1;
        let b2 = b.2;
        f32::abs(a0 - b0) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a0), f32::abs(b0)))
            && f32::abs(a1 - b1) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a1), f32::abs(b1)))
            && f32::abs(a2 - b2) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a2), f32::abs(b2)))
    }

    /**
     * Alias for {@link vec3.subtract}
     * @function
     */
    pub fn sub(out: &mut Vector3, a: &Vector3, b: &Vector3) {
        Vector3::subtract(out, a, b);
    }

    /**
     * Alias for {@link vec3.multiply}
     * @function
     */
    pub fn mul(out: &mut Vector3, a: &Vector3, b: &Vector3) {
        Vector3::multiply(out, a, b);
    }

    /**
     * Alias for {@link vec3.divide}
     * @function
     */
    pub fn div(out: &mut Vector3, a: &Vector3, b: &Vector3) {
        Vector3::divide(out, a, b);
    }

    /**
     * Alias for {@link vec3.distance}
     * @function
     */
    pub fn dist(a: &Vector3, b: &Vector3) {
        Vector3::distance(a, b);
    }

    /**
     * Alias for {@link vec3.squaredDistance}
     * @function
     */
    pub fn sqrDist(a: &Vector3, b: &Vector3) {
        Vector3::squaredDistance(a, b);
    }

    /**
     * Alias for {@link vec3.length}
     * @function
     */
    pub fn len(a: &Vector3) -> f32 {
        Vector3::length(a)
    }

    /**
     * Alias for {@link vec3.squaredLength}
     * @function
     */
    pub fn sqrLen(a: &Vector3) -> f32 {
        Vector3::squaredLength(a)
    }
}
