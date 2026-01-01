fn main() {
    loop{
        let job = fetch_job_from_queue();
        run_job(job);
    }
}