#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaySqlExecNode {
    #[prost(oneof = "ray_sql_exec_node::PlanType", tags = "3, 4")]
    pub plan_type: ::core::option::Option<ray_sql_exec_node::PlanType>,
}
/// Nested message and enum types in `RaySqlExecNode`.
pub mod ray_sql_exec_node {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PlanType {
        #[prost(message, tag = "3")]
        RayShuffleReader(super::RayShuffleReaderExecNode),
        #[prost(message, tag = "4")]
        RayShuffleWriter(super::RayShuffleWriterExecNode),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RayShuffleReaderExecNode {
    /// stage to read from
    #[prost(uint32, tag = "1")]
    pub stage_id: u32,
    /// schema of the shuffle stage
    #[prost(message, optional, tag = "2")]
    pub schema: ::core::option::Option<::datafusion_proto::protobuf::Schema>,
    /// this must match the output partitioning of the writer we are reading from
    #[prost(message, optional, tag = "3")]
    pub partitioning: ::core::option::Option<
        ::datafusion_proto::protobuf::PhysicalHashRepartition,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RayShuffleWriterExecNode {
    /// stage that is writing the shuffle files
    #[prost(uint32, tag = "1")]
    pub stage_id: u32,
    /// plan to execute
    #[prost(message, optional, tag = "2")]
    pub plan: ::core::option::Option<::datafusion_proto::protobuf::PhysicalPlanNode>,
    /// output partitioning schema
    #[prost(message, optional, tag = "3")]
    pub partitioning: ::core::option::Option<
        ::datafusion_proto::protobuf::PhysicalHashRepartition,
    >,
}
