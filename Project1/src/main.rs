use tokio::sync::{broadcast, mpsc};
#[allow(unused_imports)]
use crossbeam_channel::{unbounded, Sender};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

type EnrollmentCounts = Arc<Mutex<HashMap<Course, u32>>>;

struct TaskResult {
    task_id: String,
    success: bool,
    result_message: String
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
enum Course {
    EECS101,
    EECS102,
    EECS201,
    EECS301,
    EECS303
}

impl Course {
    fn prerequisites(&self) -> Vec<Course> {
        match self {
            Course::EECS101 => vec![],
            Course::EECS102 => vec![Course::EECS101],
            Course::EECS201 => vec![Course::EECS101, Course::EECS102],
            Course::EECS301 => vec![Course::EECS201],
            Course::EECS303 => vec![Course::EECS201],
        }
    }

    fn max_seats(&self) -> u32 {
        match self {
            Course::EECS101 => 100,
            Course::EECS102 => 80,
            Course::EECS201 => 60,
            Course::EECS301 => 40,
            Course::EECS303 => 30,
        }
    }
}

// =============== PART 1: DATA STRUCTURES & FUNCTIONS =========================

#[derive(Clone, Debug)]
struct EnrollmentTask {
    // TODO: Define the fields for EnrollmentTask
}

#[derive(Clone, Debug)]
enum EnrollmentAction {
    // TODO: Define the variants for EnrollmentAction
}


async fn process_enrollment_task(
    task: &EnrollmentTask, 
    enrollment_counts: EnrollmentCounts) -> Result<String, String> {
    
    Ok(("Processing task").to_string()) // TODO: Placeholder implementation, replace with actual logic
}

fn create_sample_tasks() -> Vec<EnrollmentTask> {
    Vec::new() // TODO: Placeholder implementation, replace with actual logic to create sample tasks
}


// ================ PART 2: BROADCAST PATTERN =========================
struct EnrollmentProcessor{
    tx: broadcast::Sender<EnrollmentTask>,
}

impl EnrollmentProcessor {
    fn new(num_workers: usize) -> (Self, mpsc::UnboundedReceiver<TaskResult>) {
        let (tx, _rx) = broadcast::channel::<EnrollmentTask>(100);
        let (result_tx, result_rx) = mpsc::unbounded_channel::<TaskResult>();
        
        //Initialize enrollment counts for each course
       let counts: HashMap<Course, u32> = [Course::EECS101, Course::EECS102, Course::EECS201, Course::EECS301, Course::EECS303]
    .into_iter()
    .map(|course| (course, 0))
    .collect();

    let enrollment_counts = Arc::new(Mutex::new(counts));

       //Spawn workers
        for _ in 0..num_workers {
            let mut task_rx = tx.subscribe();
            let result_tx = result_tx.clone();
            let enrollment_counts = enrollment_counts.clone();

            //TODO: Spawn an asynchronous task for each worker
            
        }
    (EnrollmentProcessor {tx}, result_rx)
    }

    fn submit_task(&self, task: EnrollmentTask) -> Result<(), String> {
        self.tx.send(task)
        .map(|_| ())
        .map_err(|e| format!("Failed to submit task: {}", e))
    }
}


// ================ PART 2: WORK-QUEUE PATTERN =========================
    // TODO: Implement the same EnrollmentProcessor struct and methods using a work-queue pattern with crossbeam channels.




//================ MAIN FUNCTION - DO NOT MODIFY=========================
// The main function to run the program. 
// You should not modify this function, but you can add additional functions if needed.
#[tokio::main]
async fn main() {

    // Create a task processor
    let (processor, mut result_rx) = EnrollmentProcessor::new(4);

    //Spawn a task to listen for results
    tokio::spawn(async move {
        while let Some(result) = result_rx.recv().await {
            println!(
                "Task {}: {} {}",
                result.task_id,
                result.result_message,
                if result.success { "✓" } else { "✗" }
            )
        }
    });

    //Create sample tasks
    let tasks = create_sample_tasks();
    for task in tasks {
        processor.submit_task(task).ok();
    }

    //Keep the program running to allow async tasks to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
 
    #[test]
    fn test_course_variants_exist() {
        let courses = vec![
            Course::EECS101,
            Course::EECS102,
            Course::EECS201,
            Course::EECS301,
            Course::EECS303,
        ];
        assert_eq!(courses.len(), 5);
    }
 
    #[test]
    fn test_course_prerequisites_zero() {
        assert_eq!(Course::EECS101.prerequisites(), vec![]);
    }
 
    #[test]
    fn test_course_prerequisites_one() {
        assert_eq!(Course::EECS102.prerequisites(), vec![Course::EECS101]);
        assert_eq!(Course::EECS301.prerequisites(), vec![Course::EECS201]);
        assert_eq!(Course::EECS303.prerequisites(), vec![Course::EECS201]);
    }
 
    #[test]
    fn test_course_prerequisites_multiple() {
        let prereqs = Course::EECS201.prerequisites();
        assert_eq!(prereqs.len(), 2);
        assert!(prereqs.contains(&Course::EECS101));
        assert!(prereqs.contains(&Course::EECS102));
    }
 
    #[test]
    fn test_course_max_seats() {
        assert_eq!(Course::EECS101.max_seats(), 100);
        assert_eq!(Course::EECS102.max_seats(), 80);
        assert_eq!(Course::EECS201.max_seats(), 60);
        assert_eq!(Course::EECS301.max_seats(), 40);
        assert_eq!(Course::EECS303.max_seats(), 30);
    }
 
    #[test]
    fn test_enrollment_task_creation() {
        let task = EnrollmentTask {
            id: "task-1".to_string(),
            action: EnrollmentAction::Register,
            student_id: "S001".to_string(),
            course_id: Course::EECS101,
            prereqs_taken: vec![],
        };
 
        assert_eq!(task.id, "task-1");
        assert_eq!(task.student_id, "S001");
        assert_eq!(task.course_id, Course::EECS101);
        assert!(matches!(task.action, EnrollmentAction::Register));
        assert_eq!(task.prereqs_taken.len(), 0);
    }
 
    #[test]
    fn test_all_enrollment_actions() {
        let actions = vec![
            EnrollmentAction::Register,
            EnrollmentAction::Drop,
            EnrollmentAction::PrerequisiteCheck,
            EnrollmentAction::EnrollmentStatus,
            EnrollmentAction::CanEnroll,
        ];
        assert_eq!(actions.len(), 5);
    }
 
    #[tokio::test]
    async fn test_prerequisite_check_satisfied() {
        let task = EnrollmentTask {
            id: "task-1".to_string(),
            action: EnrollmentAction::PrerequisiteCheck,
            student_id: "S001".to_string(),
            course_id: Course::EECS301,
            prereqs_taken: vec![Course::EECS201],
        };
 
        assert!(process_enrollment_task(&task, Arc::new(Mutex::new(HashMap::new()))).await.is_ok());
    }
 
    #[tokio::test]
    async fn test_prerequisite_check_unsatisfied() {
        let task = EnrollmentTask {
            id: "task-2".to_string(),
            action: EnrollmentAction::PrerequisiteCheck,
            student_id: "S002".to_string(),
            course_id: Course::EECS301,
            prereqs_taken: vec![],
        };

        assert!(process_enrollment_task(&task, Arc::new(Mutex::new(HashMap::new()))).await.is_err());
    }
 
    #[tokio::test]
    async fn test_prerequisite_check_partial() {
        let task = EnrollmentTask {
            id: "task-3".to_string(),
            action: EnrollmentAction::PrerequisiteCheck,
            student_id: "S003".to_string(),
            course_id: Course::EECS201,
            prereqs_taken: vec![Course::EECS101],
        };
 
        assert!(process_enrollment_task(&task, Arc::new(Mutex::new(HashMap::new()))).await.is_err());
    }
 
    #[test]
    fn test_enrollment_counts_initialization() {
        let counts: HashMap<Course, u32> = [
            Course::EECS101,
            Course::EECS102,
            Course::EECS201,
            Course::EECS301,
            Course::EECS303,
        ]
        .into_iter()
        .map(|course| (course, 0))
        .collect::<HashMap<Course, u32>>();
 
        assert_eq!(counts.len(), 5);
        for (_, count) in counts.iter() {
            assert_eq!(*count, 0);
        }
    }
 
    #[test]
    fn test_enrollment_counts_as_arc_mutex() {
        let enrollment_counts = Arc::new(Mutex::new(
            [Course::EECS101, Course::EECS102, Course::EECS201, Course::EECS301, Course::EECS303]
                .into_iter()
                .map(|course| (course, 0))
                .collect::<HashMap<Course, u32>>(),
        ));
 
        // Lock and read
        let counts = enrollment_counts.lock().unwrap();
        let eecs301_count = counts.get(&Course::EECS301).copied().unwrap_or(0);
        assert_eq!(eecs301_count, 0);
    }
 
    #[test]
    fn test_enrollment_counts_can_be_cloned() {
        let enrollment_counts: EnrollmentCounts = Arc::new(Mutex::new(
            [Course::EECS101, Course::EECS102, Course::EECS201, Course::EECS301, Course::EECS303]
                .into_iter()
                .map(|course| (course, 0))
                .collect(),
        ));
 
        let _cloned = enrollment_counts.clone();
        assert_eq!(Arc::strong_count(&enrollment_counts), 2);
    }
 
    #[test]
    fn test_register_with_available_seats() {
        let counts = Arc::new(Mutex::new(
            [Course::EECS301].into_iter().map(|c| (c, 25)).collect::<HashMap<Course, u32>>(),
        ));
        let current = counts.lock().unwrap().get(&Course::EECS301).copied();
        let max = Course::EECS301.max_seats();
 
        assert!(current.unwrap_or(0) < max);
    }
 
    #[test]
    fn test_register_with_no_seats() {
        let counts = Arc::new(Mutex::new(
            [Course::EECS301].into_iter().map(|c| (c, 40)).collect::<HashMap<Course, u32>>(),
        ));
        let current = counts.lock().unwrap().get(&Course::EECS301).copied();
        let max = Course::EECS301.max_seats();
 
        assert!(current.unwrap_or(0) >= max);
    }
 
    #[test]
    fn test_increment_enrollment_count() {
        let counts = Arc::new(Mutex::new(
            [Course::EECS301].into_iter().map(|c| (c, 25)).collect::<HashMap<Course, u32>>(),
        ));
 
        {
            let mut c = counts.lock().unwrap();
            let current = c.get(&Course::EECS301).copied().unwrap_or(0);
            c.insert(Course::EECS301, current + 1);
        }
 
        let c = counts.lock().unwrap();
        assert_eq!(c.get(&Course::EECS301).copied().unwrap_or(0), 26);
    }
 
    #[test]
    fn test_decrement_enrollment_count() {
        let counts = Arc::new(Mutex::new(
            [Course::EECS301].into_iter().map(|c| (c, 25)).collect::<HashMap<Course, u32>>(),
        ));
 
        {
            let mut c = counts.lock().unwrap();
            let current = c.get(&Course::EECS301).copied().unwrap_or(0);
            if current > 0 {
                c.insert(Course::EECS301, current - 1);
            }
        }
 
        let c = counts.lock().unwrap();
        assert_eq!(c.get(&Course::EECS301).copied().unwrap_or(0), 24);
    }
 
    #[test]
    fn test_enrollment_status_empty_course() {
        let counts = Arc::new(Mutex::new(
            [Course::EECS301].into_iter().map(|c| (c, 0)).collect::<HashMap<Course, u32>>(),
        ));
 
        let c = counts.lock().unwrap();
        let current = c.get(&Course::EECS301).copied().unwrap_or(0);
        let max = Course::EECS301.max_seats();
 
        assert_eq!(format!("{}/{} seats filled", current, max), "0/40 seats filled");
    }
 
    #[test]
    fn test_enrollment_status_partial() {
        let counts = Arc::new(Mutex::new(
            [Course::EECS301].into_iter().map(|c| (c, 20)).collect::<HashMap<Course, u32>>(),
        ));
 
        let c = counts.lock().unwrap();
        let current = c.get(&Course::EECS301).copied().unwrap_or(0);
        let max = Course::EECS301.max_seats();
 
        assert_eq!(format!("{}/{} seats filled", current, max), "20/40 seats filled");
    }
 
    #[test]
    fn test_enrollment_status_full() {
        let counts = Arc::new(Mutex::new(
            [Course::EECS301].into_iter().map(|c| (c, 40)).collect::<HashMap<Course, u32>>(),
        ));
 
        let c = counts.lock().unwrap();
        let current = c.get(&Course::EECS301).copied().unwrap_or(0);
        let max = Course::EECS301.max_seats();
 
        assert_eq!(format!("{}/{} seats filled", current, max), "40/40 seats filled");
    }
 
    #[test]
    fn test_task_result_success() {
        let result = TaskResult {
            task_id: "task-1".to_string(),
            success: true,
            result_message: "Student registered".to_string(),
        };
 
        assert!(result.success);
        assert!(!result.result_message.is_empty());
    }
 
    #[test]
    fn test_task_result_failure() {
        let result = TaskResult {
            task_id: "task-2".to_string(),
            success: false,
            result_message: "Course is full".to_string(),
        };
 
        assert!(!result.success);
        assert!(result.result_message.contains("full"));
    }

}