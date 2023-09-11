import logging

from openff.qcsubmit.results import OptimizationResultCollection

logging.getLogger("openff").setLevel(logging.ERROR)


opt = OptimizationResultCollection.parse_file("../testfiles/core-opt.json")

opt.to_records()
