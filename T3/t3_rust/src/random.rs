/// 随机数生成器结构体
pub struct RandomGenerator {
    seed: u64,
}

impl RandomGenerator {
    /// 创建一个新的随机数生成器实例
    /// 
    /// # 参数
    /// * `seed` - 随机数生成器的种子值
    pub fn new(seed: u64) -> Self {
        RandomGenerator { seed }
    }

    /// 生成下一个随机数
    fn next(&mut self) -> u64 {
        // 线性同余生成器的参数
        const A: u64 = 6364136223846793005;
        const C: u64 = 1;
        const M: u64 = 1 << 63;
        
        self.seed = (A.wrapping_mul(self.seed).wrapping_add(C)) % M;
        self.seed
    }

    /// 生成指定范围内的随机整数
    /// 
    /// # 参数
    /// * `min` - 范围的最小值（包含）
    /// * `max` - 范围的最大值（包含）
    /// 
    /// # 返回值
    /// 返回指定范围内的随机整数
    pub fn generate_int(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min + 1) as u64;
        let random = self.next() % range;
        min + random as i32
    }

    /// 生成指定范围内的随机浮点数
    /// 
    /// # 参数
    /// * `min` - 范围的最小值（包含）
    /// * `max` - 范围的最大值（包含）
    /// 
    /// # 返回值
    /// 返回指定范围内的随机浮点数
    pub fn generate_float(&mut self, min: f64, max: f64) -> f64 {
        let random = self.next() as f64 / u64::MAX as f64;
        min + random * (max - min)
    }

    /// 生成随机布尔值
    /// 
    /// # 返回值
    /// 返回随机的布尔值（true 或 false）
    pub fn generate_bool(&mut self) -> bool {
        self.next() % 2 == 0
    }
}
