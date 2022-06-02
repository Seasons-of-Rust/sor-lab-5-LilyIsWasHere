use personnel::Candidate;
use personnel::AstronautJob;

fn main() {
    let mut candidates: Vec<Candidate> = Candidate::load_candidate_file();
    candidates.sort_by(|a, b| get_candidate_score(a).cmp(&get_candidate_score(b)));
}

fn get_candidate_score(candidate: &Candidate) -> u32 {
    let job_score = get_job_score(candidate);
    let candidate_score = ((job_score + candidate.health as u32) * candidate.age as u32) % 3928;
    candidate_score
}

fn get_job_score(candidate: &Candidate) -> u32 {
    if let Some(secondary_job) = &candidate.secondary_job {
        get_job_code(&candidate.primary_job) * get_job_code(&secondary_job) % 576
    }
    else {
        get_job_code(&candidate.primary_job) * get_job_code(&candidate.primary_job) % 576
    }
}

fn get_job_code(job: &AstronautJob) -> u32 {
    match job {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283
    }
}


#[test]
fn test_get_job_code() {
    assert_eq!(get_job_code(&AstronautJob::Biogeochemist), 251);
    assert_eq!(get_job_code(&AstronautJob::Biologist), 257);
    assert_eq!(get_job_code(&AstronautJob::Engineer), 263);
    assert_eq!(get_job_code(&AstronautJob::Geologist), 269);
    assert_eq!(get_job_code(&AstronautJob::Mechanic), 271);
    assert_eq!(get_job_code(&AstronautJob::Medic), 277);
    assert_eq!(get_job_code(&AstronautJob::RoverOp), 281);
    assert_eq!(get_job_code(&AstronautJob::Scientist), 283);
}

#[test]
fn test_get_job_score() {
    let candidate = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Biologist),
        age: 30,
        health: 100
    };
    assert_eq!(get_job_score(&candidate), get_job_code(&AstronautJob::Biogeochemist) * get_job_code(&AstronautJob::Biologist) % 576);
}

#[test]
fn test_get_candidate_score() {
    let candidate = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Biologist),
        age: 30,
        health: 100
    };
    assert_eq!(get_candidate_score(&candidate), ((get_job_score(&candidate) + 100) * 30) % 3928);
}

